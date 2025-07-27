use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    text::Line,
    widgets::{Block, BorderType, LineGauge, Widget},
};

use crate::app::App;

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Max(3)])
            .split(area);

        let block = Block::bordered()
            .title(format!("{}/{}", self.context.clone(), self.machine))
            .title(
                Line::from(Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false))
                    .left_aligned(),
            )
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain);

        LineGauge::default()
            .ratio(0.4)
            .label("0:30|4:32")
            .block(
                Block::bordered()
                    .title("Track...")
                    .border_type(BorderType::Plain),
            )
            .render(layout[1].inner(Margin::default()), buf);

        // Render menus
        self.machine
            .render(block.inner(layout[0].inner(Margin::default())), buf)
            .expect("Render Error: ");

        // Render blocks
        block.render(layout[0].inner(Margin::default()), buf);
    }
}

struct EditDevicePopup {
    // device
}
