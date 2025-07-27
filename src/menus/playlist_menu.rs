use std::fmt::Display;

use crate::{machine::Instruction, playlist::Playlist};

use super::menu::{Menu, MenuState};
use ratatui::{
    prelude::*,
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

pub struct PlaylistMenu {
    pub state: TableState,
    pub playlists: Vec<Playlist>,
}

impl Display for PlaylistMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("playlist")
    }
}

impl Menu for PlaylistMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        self.playlists.len()
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into(), Instruction::Continue.into()]
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);

        let header = ["Title", "Tracks", "Duration"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let table = Table::new(
            self.playlists.iter(),
            [Constraint::Min(5), Constraint::Max(2), Constraint::Max(5)],
        )
        .header(header)
        .highlight_symbol(">")
        .highlight_spacing(if focused {
            HighlightSpacing::Always
        } else {
            HighlightSpacing::Never
        });

        StatefulWidget::render(table, area, buf, &mut self.state);
        Ok(area)
    }
}

impl PlaylistMenu {
    pub fn new(playlists: Vec<Playlist>) -> Self {
        let mut state = TableState::new();
        state.select_next();
        Self { state, playlists }
    }
}
