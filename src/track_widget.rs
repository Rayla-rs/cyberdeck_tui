use ratatui::{
    text::Text,
    widgets::{Cell, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::track::Track;

pub struct TrackWidget {
    state: TableState,
    tracks: Vec<Track>,
}

impl TrackWidget {
    pub fn up(&mut self) {
        self.state.select_next_column();
    }

    pub fn down(&mut self) {
        self.state.select_previous_column();
    }

    pub fn enter(&mut self) {
        todo!()
    }
}

impl Widget for &mut TrackWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let header = [" ", "Title", "Artist", "Duration"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.tracks.iter().enumerate().map(|(index, track)| {
            vec![index.to_string()]
                .iter()
                .chain(track.data().iter())
                .map(|datum| Cell::from(Text::from(format!("\n{:?}\n", datum))))
                .collect::<Row>()
        });

        // TODO create constraints

        let table = Table::new(rows, [1, 4, 4, 4]).header(header);
        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}
