use std::fmt::Display;

use ratatui::{
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, HighlightSpacing, List, ListState},
};

use crate::{AppResult, app_actions::AppAction, machine::Instruction, menus::menu::Menu};

pub struct QuickMenu {
    pub actions: Vec<AppAction>,
    pub state: ListState,
}

impl Display for QuickMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("quick_menu")
    }
}

impl Menu for QuickMenu {
    fn get_state(&mut self) -> &mut dyn crate::menus::menu::MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        self.actions.len()
    }

    fn enter(&mut self) -> AppResult<AppAction> {
        let index = self.state.selected().unwrap();
        Ok(self.actions.remove(index))
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> AppResult<Rect>
    where
        Self: Sized,
    {
        let list = List::default()
            .block(Block::new().borders(Borders::TOP))
            .items(self.actions.iter())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            });

        StatefulWidget::render(list, area, buf, &mut self.state);
        Ok(area)
    }
}

impl QuickMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select_next();
        Self {
            state: ListState::default(),
            actions: vec![AppAction::MachineAction(Instruction::Pop)],
        }
    }

    pub fn get_height(&self) -> usize {
        if self.actions.is_empty() {
            0
        } else {
            self.actions.len() + 1
        }
    }
}
