use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Table, Widget},
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
        let block = Block::bordered()
            .title(self.context.clone())
            .title(
                Line::from(Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false))
                    .left_aligned(),
            )
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain);

        self.machine
            .render(block.inner(area), buf)
            .expect("Render Error: ");
        block.render(area, buf);
    }
}

struct EditDevicePopup {
    // device
}
