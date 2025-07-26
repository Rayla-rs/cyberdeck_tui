use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        Bar, Block, BorderType, LineGauge, Paragraph, Scrollbar, ScrollbarState, StatefulWidget,
        Table, Widget,
    },
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
            .constraints([Constraint::Length(5), Constraint::Min(1)])
            .split(area);

        let block = Block::bordered()
            .title(self.context.clone())
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
