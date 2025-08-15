use std::{fmt::Debug, sync::Arc};

use color_eyre::eyre::OptionExt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Cell, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget},
};
// TODO:
// Stacking menu
// Async menu
// Unbounded events
// Info Menu (Not items just paragraph or data)

use crate::{
    CONFIG,
    app::{AppState, quick_menu},
    device::BluetoothItem,
    event::AppEvent,
};

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

    pub fn is_leaf(&self) -> bool {
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

    fn tick(&mut self, app_state: &AppState) -> color_eyre::Result<()> {
        match self.next.as_mut() {
            Some(next) => next.tick(app_state),
            None => self.current.tick(app_state),
        }
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
    pub fn new(menus: [Box<dyn Menu>; N]) -> Self {
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

    fn tick(&mut self, app_state: &AppState) -> color_eyre::Result<()> {
        for menu in self.menus.as_mut() {
            menu.tick(app_state)?;
        }
        Ok(())
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
    header: Option<Row<'static>>,
    ticker: Option<fn(&mut Vec<T>, &AppState) -> color_eyre::Result<()>>,
    state: TableState,
}

impl<T, C> TableMenu<T, C>
where
    T: Item,
    C: IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    pub fn new(items: Vec<T>, widths: C) -> Self {
        Self {
            items,
            widths,
            header: None,
            ticker: None,
            state: TableState::default(),
        }
    }

    pub fn with_header(mut self, header: Row<'static>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_ticker(
        mut self,
        ticker: fn(&mut Vec<T>, &AppState) -> color_eyre::Result<()>,
    ) -> Self {
        self.ticker = Some(ticker);
        self
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

        StatefulWidget::render(
            Table::new(self.items.clone().into_iter(), self.widths.clone())
                .highlight_symbol(">>".yellow())
                .highlight_spacing(HighlightSpacing::Always),
            if let Some(header) = self.header.as_ref() {
                let layout =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(100)]).split(area);
                Widget::render(
                    Table::default()
                        .widths(self.widths.clone())
                        .header(header.clone()),
                    layout[0],
                    buf,
                );
                layout[1]
            } else {
                area
            },
            buf,
            &mut self.state,
        );
    }

    fn constraint(&self) -> Constraint {
        Constraint::Length(self.items.len() as u16 + if self.header.is_some() { 1 } else { 0 })
    }

    fn tick(&mut self, app_state: &AppState) -> color_eyre::Result<()> {
        match self.ticker.as_mut() {
            Some(ticker) => ticker(&mut self.items, app_state),
            None => Ok(()),
        }
    }
}

pub fn make_test_menu() -> LinkedMenu {
    LinkedMenu::new(Box::new(MenuFrame::new([
        Box::new(TextMenu(
            Text::from(
                r"Nokota V1
Copyright (c) Rayla-rs <wassrayla@gmail.com>
            ",
            )
            .centered(),
        )),
        Box::new(PlaylistItem.to_menu()),
        Box::new(BluetoothItem.to_menu()),
        quick_menu(),
    ])))
}

#[derive(Clone)]
pub struct PlaylistItem;

impl PlaylistItem {
    pub fn to_menu(self) -> TableMenu<PlaylistItem, [Constraint; 1]> {
        TableMenu::new(vec![self], [Constraint::Fill(100)])
    }
}

impl Into<AppEvent> for PlaylistItem {
    fn into(self) -> AppEvent {
        AppEvent::Push(Arc::new(|| playlist_collection_menu()))
    }
}

impl<'a> Into<Row<'a>> for PlaylistItem {
    fn into(self) -> Row<'a> {
        Row::new([Cell::new("PlaylistMenu")])
    }
}

impl Item for PlaylistItem {}

pub fn playlist_collection_menu() -> LinkedMenu {
    LinkedMenu::new(Box::new(MenuFrame::new([
        Box::new(
            TableMenu::new(
                CONFIG.load_playlists().collect(),
                [
                    Constraint::Min(5),
                    Constraint::Length(6),
                    Constraint::Length(8),
                ],
            )
            .with_header(Row::new([
                Cell::new("Title"),
                Cell::new("Tracks"),
                Cell::new("Duration"),
            ])),
        ),
        quick_menu(),
    ])))
}

pub struct TextMenu(pub Text<'static>);

impl Menu for TextMenu {
    fn up(&mut self) -> NavigationResult {
        NavigationResult::Previous
    }

    fn down(&mut self) -> NavigationResult {
        NavigationResult::Next
    }

    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        Ok(None)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, _focused: bool) {
        self.0.clone().render(area, buf);
    }

    fn constraint(&self) -> Constraint {
        Constraint::Length(self.0.lines.len() as u16)
    }
}
