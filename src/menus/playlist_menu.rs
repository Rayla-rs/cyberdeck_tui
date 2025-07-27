use std::fmt::Display;

use crate::{
    machine::Instruction, playlist::Playlist, track_widget::TrackWidget,
    widgets::playlist_widget::PlaylistWidget,
};

use super::menu::Menu;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListState, Paragraph},
};

// Back
// Play
// Pause
// ...
struct LowerMenu {
    state: ListState,
    stuff: Vec<()>,
}

impl LowerMenu {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            stuff: vec![],
        }
    }
}

impl Widget for &mut LowerMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // TODO hide when empty
        let list = List::default()
            .block(Block::new().borders(Borders::TOP))
            .items(["Back", "Pause", "..."])
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

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
    fn up(&mut self) {
        self.widget.up();
    }
    fn down(&mut self) {
        self.widget.down();
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
        LowerMenu::new().render(layout[1], buf);
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
