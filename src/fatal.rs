use std::process::Termination;

use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use crossterm::terminal;
use futures::StreamExt;
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Paragraph, Widget, Wrap},
};

pub struct FatalWidget(pub color_eyre::Report);

impl Widget for &FatalWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(9),
            Constraint::Percentage(50),
        ])
        .split(area);

        Block::default().on_blue().render(area, buf);

        // Text::from("NOKOTA").blue().on_white().centered().render(
        //     Layout::horizontal([
        //         Constraint::Fill(50),
        //         Constraint::Length(6),
        //         Constraint::Fill(50),
        //     ])
        //     .split(layout[1])[1],
        //     buf,
        // );

        Text::from("FATAL ERROR")
            .red()
            .on_blue()
            .centered()
            .render(inner_horizontal(layout[2]), buf);
        Paragraph::new(Text::from(format!(
            r"
            A FATAL ERROR HAS OCCURED AT {} IN NOCOTA-SYS-RXGM8-HLDJB.
            IF THIS PROBLEM PERSISTS, CONTACT RAYLA-RS.
            REPORT {}


            PRESS ANY KEY TO REBOOT NOKOTA LOGIC",
            Local::now().to_rfc3339(),
            self.0
        )))
        .centered()
        .wrap(Wrap { trim: true })
        .white()
        .on_blue()
        .render(inner_horizontal(layout[3]), buf);
    }
}

fn inner_horizontal(area: Rect) -> Rect {
    Layout::horizontal([
        Constraint::Percentage(12),
        Constraint::Percentage(76),
        Constraint::Percentage(12),
    ])
    .split(area)[1]
}

impl FatalWidget {
    pub async fn run(self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
        crossterm::event::EventStream::new().next().await;
        Ok(())
    }
}
