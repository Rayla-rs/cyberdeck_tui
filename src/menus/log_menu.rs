use std::{fmt::Display, fs, io::BufRead, ops::Deref, path::PathBuf};

use crate::{logging, machine::Instruction};

use super::menu::{Menu, MenuState};
use ratatui::{
    prelude::*,
    widgets::{HighlightSpacing, List, ListState},
};

#[derive(Debug)]
pub struct LogMenu {
    state: ListState,
    lines: Vec<String>,
}

impl Display for LogMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("...")
    }
}

impl Menu for LogMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        self.lines.len()
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);

        // Layout::new(direction, constraints)

        // paragraph of data

        // List of actions
        let list = List::new(self.lines.clone())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
            .yellow();
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);

        Ok(area)
    }
}

impl LogMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self {
            state,
            lines: logging::read_all_lines(),
        }
    }
}
