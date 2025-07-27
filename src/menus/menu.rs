use std::fmt::Display;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, TableState},
};

use crate::{AppResult, app::AppState, machine::Instruction};

pub enum NavigationResult {
    Ok,
    Underflow,
    Overflow,
}

pub trait Menu: Display {
    fn get_state(&mut self) -> &mut dyn MenuState;

    fn get_len(&self) -> usize;

    fn up(&mut self) -> NavigationResult {
        let state = self.get_state();
        if let Some(selected) = state.selected() {
            if selected == 0 {
                state.select(None);
                NavigationResult::Underflow
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
                state.select(None);
                NavigationResult::Overflow
            } else {
                state.select_next();
                NavigationResult::Ok
            }
        } else {
            state.select_next();
            NavigationResult::Ok
        }
    }

    fn enter(&mut self) -> AppResult<Instruction> {
        Ok(Instruction::Continue)
    }
    fn tick(&mut self, _app_state: &mut AppState) -> AppResult<Instruction> {
        Ok(Instruction::Continue)
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer) -> AppResult<Rect>;
}

pub trait MenuState {
    fn select(&mut self, index: Option<usize>);
    fn selected(&self) -> Option<usize>;
    fn select_next(&mut self);
    fn select_previous(&mut self);
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
}
