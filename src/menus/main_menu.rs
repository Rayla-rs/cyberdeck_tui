use std::fmt::Display;

use ratatui::{
    style::Stylize,
    widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};
use strum::{EnumCount, IntoEnumIterator, VariantArray};
use strum_macros::{Display, EnumCount, EnumIter, VariantArray};

use crate::{AppResult, CONFIG, app_actions::AppAction, machine::Instruction, menus::menu::Menu};

use super::{
    blt_menu::BltMenu, log_menu::LogMenu, menu::MenuState,
    playlist_collection_menu::PlaylistCollectionMenu,
};

#[derive(Display, EnumIter, VariantArray, EnumCount)]
enum Options {
    Music,
    Wifi,
    Bluetooth,
    Log,
    Reboot,
}

impl<'a> Into<ListItem<'a>> for Options {
    fn into(self) -> ListItem<'a> {
        ListItem::from(format!("{}", self))
    }
}

#[derive(Debug)]
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
        Options::COUNT
    }

    fn enter(&mut self) -> AppResult<AppAction> {
        Ok(match Options::VARIANTS
            .get(self.state.selected().ok_or("Selection empty")?)
            .ok_or("Index out of bounds in Main Menu!")?
        {
            Options::Reboot => Instruction::Pop,
            Options::Music => Instruction::Push(Box::new(PlaylistCollectionMenu::new(
                CONFIG.load_playlists().collect(),
            ))),
            Options::Bluetooth => Instruction::Push(Box::new(BltMenu::new())),
            Options::Log => Instruction::Push(Box::new(LogMenu::new())),
            _ => Instruction::Continue,
        }
        .into())
    }

    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focused: bool,
    ) -> AppResult<ratatui::prelude::Rect> {
        let list = List::new(Options::iter())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
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
