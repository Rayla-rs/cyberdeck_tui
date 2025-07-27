use std::fmt::Display;

use ratatui::{
    style::Stylize,
    widgets::{List, ListItem, ListState, StatefulWidget},
};

use crate::{AppResult, config::Config, machine::Instruction, menus::menu::Menu};

use super::{
    menu::{MenuState, NavigationResult},
    playlist_menu::PlaylistMenu,
};

enum Options {
    Music,
    Wifi,
    Bluetooth,
    Reboot,
}
// Add menu to look at launch errs

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Music => "Music",
            Self::Wifi => "Wifi",
            Self::Bluetooth => "Bluetooth",
            Self::Reboot => "Reboot",
        })
    }
}

impl<'a> Into<ListItem<'a>> for Options {
    fn into(self) -> ListItem<'a> {
        ListItem::from(format!("{}", self))
    }
}

const OPTIONS: [Options; 4] = [
    Options::Music,
    Options::Wifi,
    Options::Bluetooth,
    Options::Reboot,
];

pub struct MainMenu {
    state: ListState,
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("main")
    }
}

impl Menu for MainMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        OPTIONS.len()
    }

    fn enter(&mut self) -> AppResult<Instruction> {
        Ok(
            match OPTIONS
                .get(self.state.selected().ok_or("Selection empty")?)
                .ok_or("Index out of bounds in Main Menu!")?
            {
                Options::Reboot => Instruction::Pop,
                Options::Music => Instruction::Push(Box::new(PlaylistMenu::new(
                    Config::new().unwrap().load_playlists().collect(),
                ))),
                _ => Instruction::Continue,
            },
        )
    }

    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) -> AppResult<ratatui::prelude::Rect> {
        let list = List::new(OPTIONS)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .yellow();
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);
        Ok(area)
    }
}

impl MainMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self { state }
    }
}
