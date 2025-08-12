use std::marker::PhantomData;

use color_eyre::eyre::OptionExt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Cell, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget},
};

// TODO:
// Stacking menu
// Async menu
// Unbounded events
// Info Menu (Not items just paragraph or data)

use crate::event::AppEvent;

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

    // TODO: fn tick(&mut self, app_state: AppState);
}

pub struct StackingMenu {
    current: Box<dyn Menu>,
    next: Option<Box<dyn Menu>>,
}

impl StackingMenu {
    // how do i handel pop / push events
}

//TODO move to util.rs
enum Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

/// Defines a array of one or more menues that render stacked!
pub struct MenuFrame<const N: usize>
where
    Assert<{ N > 0 }>: IsTrue,
{
    menus: [Box<dyn Menu>; N],
    selected: usize,
}

impl<const N: usize> MenuFrame<N>
where
    Assert<{ N > 0 }>: IsTrue,
{
    fn new(menus: [Box<dyn Menu>; N]) -> Self {
        Self { menus, selected: 0 }
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
                    NavigationResult::Previous
                } else {
                    self.up()
                }
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
                    NavigationResult::Next
                } else {
                    self.down()
                }
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
        let mut state = TableState::default();
        state.select_first();
        Self {
            items,
            widths,
            state,
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
                NavigationResult::Previous
            } else {
                self.state.select_previous();
                NavigationResult::Ok
            }
        } else {
            self.state.select_previous();
            NavigationResult::Ok
        }
    }

    fn down(&mut self) -> NavigationResult {
        self.state.select_next();
        if let Some(selected) = self.state.selected() {
            if selected >= self.items.len() {
                NavigationResult::Next
            } else {
                NavigationResult::Ok
            }
        } else {
            self.state.select_next();
            NavigationResult::Ok
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
            })
            .yellow()
            .block(Block::bordered());
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);
    }

    fn constraint(&self) -> Constraint {
        self.constraint
    }
}

trait Item: Into<AppEvent> + Into<Row<'static>> + Clone {}

pub fn make_test_menu() -> Box<dyn Menu> {
    let widths = [Constraint::Min(4), Constraint::Length(5)];
    let items = vec![DebugItem, DebugItem, DebugItem];
    Box::new(TableMenu::new(items, widths, Constraint::Fill(100)))
}

#[derive(Clone)]
struct DebugItem;

impl Into<AppEvent> for DebugItem {
    fn into(self) -> AppEvent {
        AppEvent::Debug
    }
}

impl Into<Row<'static>> for DebugItem {
    fn into(self) -> Row<'static> {
        Row::new([Cell::new(Text::from("asdg")), Cell::new(Text::from(":3"))])
    }
}
impl Item for DebugItem {}
