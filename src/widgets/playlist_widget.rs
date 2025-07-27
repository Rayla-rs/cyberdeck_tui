use crate::playlist::Playlist;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Row, Table, TableState, Widget};

pub struct PlaylistWidget {
    state: TableState,
    playlists: Vec<Playlist>,
}

impl PlaylistWidget {
    pub fn up(&mut self) {
        self.state.select_previous();
    }

    pub fn down(&mut self) {
        self.state.select_next();
    }

    pub fn enter(&mut self) {
        todo!()
    }

    pub fn new(playlists: Vec<Playlist>) -> Self {
        let mut state = TableState::new();
        state.select_next();
        Self { state, playlists }
    }
}

impl Widget for &mut PlaylistWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
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
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}
