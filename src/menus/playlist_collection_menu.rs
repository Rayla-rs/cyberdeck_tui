use std::fmt::Display;

use crate::{machine::Instruction, playlist::Playlist};

use super::{
    menu::{Menu, MenuState},
    playlist_menu::PlaylistMenu,
};
use ratatui::{
    prelude::*,
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

#[derive(Debug)]
pub struct PlaylistCollectionMenu {
    pub state: TableState,
    pub playlists: Vec<Playlist>,
}

impl Display for PlaylistCollectionMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("playlist")
    }
}

impl Menu for PlaylistCollectionMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        self.playlists.len()
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn enter(&mut self) -> crate::AppResult<crate::app_actions::AppAction> {
        Ok(Instruction::Push(Box::new(PlaylistMenu::new(
            self.playlists
                .get(self.state.selected().unwrap())
                .unwrap()
                .clone(),
        )))
        .into())
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

impl PlaylistCollectionMenu {
    pub fn new(playlists: Vec<Playlist>) -> Self {
        let mut state = TableState::new();
        state.select_next();
        Self { state, playlists }
    }
}
