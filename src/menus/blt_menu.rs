use std::fmt::Display;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{ListState, Widget};

use crate::machine::Instruction;

use super::menu::{Menu, MenuState};

#[derive(Debug)]
pub struct BltMenu {
    state: ListState,
}

impl Display for BltMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("blt_menu")
    }
}

impl Menu for BltMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        1
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);
        Ok(area)
    }
}

impl BltMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self { state }
    }
}
