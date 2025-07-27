use std::fmt::Display;

use crate::{
    machine::Instruction, playlist::Playlist, track_widget::TrackWidget,
    widgets::playlist_widget::PlaylistWidget,
};

use super::menu::Menu;
use ratatui::prelude::*;

pub struct PlaylistMenu {
    widget: PlaylistWidget,
    // TODO focus
}

impl Display for PlaylistMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("playlist")
    }
}

impl Menu for PlaylistMenu {
    fn get_state(&mut self) -> &mut impl super::menu::MenuState {
        &mut self.widget.state
    }

    fn get_len(&self) -> usize {
        self.widget.playlists.len()
    }

    fn enter(&mut self) -> crate::AppResult<Instruction> {
        // TODO
        Ok(Instruction::Continue)
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer) -> crate::AppResult<Rect> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(4)])
            .split(area);

        ratatui::widgets::Clear::default().render(area, buf);

        self.widget.render(layout[0], buf);

        Ok(area)
    }
}

impl PlaylistMenu {
    pub fn new(playlists: Vec<Playlist>) -> Self {
        Self {
            widget: PlaylistWidget::new(playlists),
        }
    }
}
