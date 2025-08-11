use std::fmt::{Debug, Display};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, TableState},
};

use crate::{AppResult, app::AppState, app_actions::AppAction, machine::Instruction};

pub enum NavigationResult {
    Ok,
    Previous,
    Next,
}

#[deprecated]
pub trait Menu: Debug + Display {
    fn get_state(&mut self) -> &mut dyn MenuState;

    fn get_len(&self) -> usize;

    fn up(&mut self) -> NavigationResult {
        let state = self.get_state();
        if let Some(selected) = state.selected() {
            if selected == 0 {
                NavigationResult::Previous
            } else {
                state.select_previous();
                NavigationResult::Ok
            }
        } else {
            state.select_previous();
            NavigationResult::Ok
        }
    }

    fn down(&mut self) -> NavigationResult {
        let len = self.get_len();
        let state = self.get_state();

        state.select_next();
        if let Some(selected) = state.selected() {
            if selected >= len {
                NavigationResult::Next
            } else {
                NavigationResult::Ok
            }
        } else {
            state.select_next();
            NavigationResult::Ok
        }
    }

    fn enter(&mut self) -> AppResult<AppAction> {
        Ok(Instruction::Continue.into())
    }

    fn get_quick_actions(&self) -> Vec<AppAction> {
        vec![]
    }

    fn tick(&mut self, _app_state: &mut AppState) -> AppResult<Instruction> {
        Ok(Instruction::Continue)
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> AppResult<Rect>;
}

pub trait MenuState {
    fn select(&mut self, index: Option<usize>);
    fn selected(&self) -> Option<usize>;
    fn select_next(&mut self);
    fn select_previous(&mut self);
    fn select_first(&mut self);
    fn select_last(&mut self);
}

impl MenuState for ListState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }
    fn selected(&self) -> Option<usize> {
        self.selected()
    }
    fn select_next(&mut self) {
        self.select_next();
    }
    fn select_previous(&mut self) {
        self.select_previous();
    }
    fn select_first(&mut self) {
        self.select_first();
    }
    fn select_last(&mut self) {
        self.select_last();
    }
}

impl MenuState for TableState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }
    fn selected(&self) -> Option<usize> {
        self.selected()
    }
    fn select_next(&mut self) {
        self.select_next();
    }
    fn select_previous(&mut self) {
        self.select_previous();
    }
    fn select_first(&mut self) {
        self.select_next();
    }
    fn select_last(&mut self) {
        self.select_last();
    }
}

#[allow(unused)]
mod test {
    use std::marker::PhantomData;

    use color_eyre::eyre::OptionExt;
    use ratatui::{
        buffer::Buffer,
        layout::{Constraint, Margin, Rect},
        style::Stylize,
        widgets::{Block, HighlightSpacing, List, Row, StatefulWidget, Table, TableState, Widget},
    };

    use crate::event::AppEvent;

    use super::{MenuState, NavigationResult};

    pub struct Menu<'a, T, C>
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

    impl<'a, T, C> Menu<'a, T, C>
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

        pub fn up(&mut self) -> NavigationResult {
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

        pub fn down(&mut self) -> NavigationResult {
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

        pub fn enter(&mut self) -> color_eyre::Result<Option<AppEvent>> {
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

        pub fn render(
            &'a mut self,
            area: Rect,
            buf: &mut Buffer,
            focused: bool,
        ) -> color_eyre::Result<Rect> {
            let area = area.inner(Margin {
                horizontal: 2,
                vertical: 2,
            });

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
}
