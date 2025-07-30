use std::fmt::Display;

use crate::{machine::Instruction, playlist::Playlist};

use super::menu::{Menu, MenuState};
use ratatui::{
    prelude::*,
    widgets::{Cell, HighlightSpacing, ListState, Row, Table, TableState},
};

pub struct PlaylistMenu {
    state: ListState,
    playlist: Playlist,
}

impl Display for PlaylistMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.playlist.title.as_str())
    }
}

impl Menu for PlaylistMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        todo!()
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        Ok(area)
    }
}
