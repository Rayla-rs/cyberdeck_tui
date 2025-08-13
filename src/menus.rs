use std::fmt::{Debug, Write};

use color_eyre::eyre::OptionExt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget},
};

// TODO:
// Stacking menu
// Async menu
// Unbounded events
// Info Menu (Not items just paragraph or data)

use crate::{app::AppState, event::AppEvent};

pub enum NavigationResult {
    Ok,
    Previous,
    Next,
}

/// Trait for implimenting stateful menu widgets!
pub trait Menu {
    fn up(&mut self) -> NavigationResult;
    fn down(&mut self) -> NavigationResult;
    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>>;
    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool);
    fn constraint(&self) -> Constraint;

    fn tick(&mut self, _app_state: &AppState) -> color_eyre::Result<()> {
        Ok(())
    }
}

pub struct LinkedMenu {
    current: Box<dyn Menu>,
    next: Option<Box<LinkedMenu>>,
}

impl Debug for LinkedMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LinkedMenu")
    }
}

impl LinkedMenu {
    pub fn new(current: Box<dyn Menu>) -> Self {
        Self {
            current,
            next: None,
        }
    }

    pub fn new_with_next(current: Box<dyn Menu>, next: LinkedMenu) -> Self {
        Self {
            current,
            next: Some(Box::new(next)),
        }
    }

    fn is_leaf(&self) -> bool {
        self.next.is_none()
    }

    pub fn push(&mut self, menu: LinkedMenu) {
        match self.next.as_mut() {
            Some(next) => next.push(menu),
            None => self.next = Some(Box::new(menu)),
        }
    }

    /// Pops the last element of the list
    ///
    /// If this is called on a leaf it does nothing
    pub fn pop(&mut self) {
        if let Some(next) = self.next.as_mut() {
            if next.is_leaf() {
                self.next = None;
            } else {
                next.pop()
            }
        }
    }
}

impl Menu for LinkedMenu {
    fn up(&mut self) -> NavigationResult {
        match self.next.as_mut() {
            Some(next) => next.up(),
            None => self.current.up(),
        }
    }

    fn down(&mut self) -> NavigationResult {
        match self.next.as_mut() {
            Some(next) => next.down(),
            None => self.current.down(),
        }
    }

    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        match self.next.as_mut() {
            Some(next) => next.enter(),
            None => self.current.enter(),
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) {
        match self.next.as_mut() {
            Some(next) => next.render(area, buf, focused),
            None => self.current.render(area, buf, focused),
        }
    }

    /// Not aplicable for this menu however in the future
    /// may be if pop and push(..) are handeled diffrently
    fn constraint(&self) -> Constraint {
        Constraint::Fill(100)
    }
}

//TODO move to util.rs
pub enum Assert<const COND: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

/// Defines a array of one or more menues that render stacked!
pub struct MenuFrame<const N: usize>
where
    Assert<{ N > 0 }>: IsTrue,
{
    menus: [Box<dyn Menu>; N],
    selected: usize,
    // TODO -> next
}

impl<const N: usize> MenuFrame<N>
where
    Assert<{ N > 0 }>: IsTrue,
{
    fn new(menus: [Box<dyn Menu>; N]) -> Self {
        let mut frame = Self { menus, selected: 0 };
        let _ = frame.down();
        frame
    }
}

impl<const N: usize> Menu for MenuFrame<N>
where
    Assert<{ N > 0 }>: IsTrue,
{
    fn up(&mut self) -> NavigationResult {
        match self.menus[self.selected].up() {
            NavigationResult::Ok => NavigationResult::Ok,
            NavigationResult::Previous => {
                if self.selected == 0 {
                    self.selected = N - 1;
                } else {
                    self.selected -= 1;
                }
                self.up()
            }
            NavigationResult::Next => {
                panic!("Unexpected Result")
            }
        }
    }

    fn down(&mut self) -> NavigationResult {
        match self.menus[self.selected].down() {
            NavigationResult::Ok => NavigationResult::Ok,
            NavigationResult::Next => {
                if self.selected == N - 1 {
                    self.selected = 0;
                } else {
                    self.selected += 1;
                }
                self.down()
            }
            NavigationResult::Previous => {
                panic!("Unexpected Result")
            }
        }
    }

    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        self.menus[self.selected].enter()
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) {
        Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(self.menus.iter().map(|menu| menu.constraint()))
            .split(area)
            .iter()
            .zip(&mut self.menus)
            .enumerate()
            .for_each(|(index, (rect, menu))| {
                menu.render(rect.clone(), buf, focused && index == self.selected)
            });
    }

    fn constraint(&self) -> Constraint {
        Constraint::Percentage(100)
    }
}

pub trait Item: Into<AppEvent> + Into<Row<'static>> + Clone {}

pub struct TableMenu<T, C>
where
    T: Item,
    C: IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    items: Vec<T>,
    widths: C,
    constraint: Constraint,
    state: TableState,
}

impl<T, C> TableMenu<T, C>
where
    T: Item,
    C: IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    pub fn new(items: Vec<T>, widths: C, constraint: Constraint) -> Self {
        Self {
            items,
            widths,
            state: TableState::default(),
            constraint,
        }
    }
}

impl<'a, T, C> Menu for TableMenu<T, C>
where
    T: Item,
    C: 'a + IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    fn up(&mut self) -> NavigationResult {
        if let Some(selected) = self.state.selected() {
            if selected == 0 {
                self.state.select(None);
                NavigationResult::Previous
            } else {
                self.state.select_previous();
                NavigationResult::Ok
            }
        } else {
            if self.items.is_empty() {
                NavigationResult::Previous
            } else {
                self.state.select_previous();
                NavigationResult::Ok
            }
        }
    }

    fn down(&mut self) -> NavigationResult {
        self.state.select_next();
        if let Some(selected) = self.state.selected() {
            if selected >= self.items.len() {
                self.state.select(None);
                NavigationResult::Next
            } else {
                NavigationResult::Ok
            }
        } else {
            if self.items.is_empty() {
                NavigationResult::Next
            } else {
                self.state.select_next();
                NavigationResult::Ok
            }
        }
    }

    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        Ok(match self.state.selected() {
            Some(index) => {
                let res = (*self.items.get(index).ok_or_eyre("Out of bounds")?)
                    .clone()
                    .into();
                Some(res)
            }
            None => None,
        })
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) {
        ratatui::widgets::Clear::default().render(area, buf);

        let list = Table::new(self.items.clone().into_iter(), self.widths.clone())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            });
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);
    }

    fn constraint(&self) -> Constraint {
        self.constraint
    }
}

pub fn make_test_menu() -> LinkedMenu {
    let widths = [Constraint::Min(4), Constraint::Length(5)];
    let items = vec![AppEvent::Up, AppEvent::Down, AppEvent::Down, AppEvent::Quit];
    LinkedMenu::new(Box::new(MenuFrame::new([
        Box::new(TableMenu::new(items, widths, Constraint::Fill(100))),
        Box::new(TableMenu::new(
            vec![AppEvent::Quit],
            widths,
            Constraint::Length(1),
        )),
    ])))
}
