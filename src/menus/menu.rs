use std::marker::PhantomData;

use color_eyre::eyre::OptionExt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    widgets::{Block, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::event::AppEvent;

pub enum NavigationResult {
    Ok,
    Previous,
    Next,
}

enum Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

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
        todo!()
    }
    fn down(&mut self) -> NavigationResult {
        todo!()
    }
    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        todo!()
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> color_eyre::Result<Rect> {
        todo!()
    }
}

pub trait Menu {
    fn up(&mut self) -> NavigationResult;
    fn down(&mut self) -> NavigationResult;
    fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>>;
    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> color_eyre::Result<Rect>;
    // todo -> get v constraint
}

pub struct TableMenu<'a, T, C>
where
    T: 'a + Into<AppEvent> + Clone,
    &'a T: Into<Row<'a>>,
    C: IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    items: Vec<T>,
    widths: C,
    state: TableState,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, C> TableMenu<'a, T, C>
where
    T: 'a + Into<AppEvent> + Clone,
    &'a T: Into<Row<'a>>,
    C: 'a + IntoIterator + Clone,
    C::Item: Into<Constraint>,
{
    pub fn new(items: Vec<T>, widths: C) -> Self {
        let mut state = TableState::default();
        state.select_first();
        Self {
            items,
            widths,
            state,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T, C> Menu for TableMenu<'a, T, C>
where
    T: 'a + Into<AppEvent> + Clone,
    &'a T: Into<Row<'a>>,
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
            Some(index) => Some(
                self.items
                    .get(index)
                    .ok_or_eyre("Out of bounds")?
                    .clone()
                    .into(),
            ),
            None => None,
        })
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> color_eyre::Result<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);

        let list = Table::new(self.items.iter(), self.widths.clone())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
            .yellow()
            .block(Block::bordered());
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);

        Ok(area)
    }
}
