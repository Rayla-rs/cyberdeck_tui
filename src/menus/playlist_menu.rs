use std::fmt::Display;

use crate::{
    app_actions::{AppAction, PlayPlaylist},
    machine::Instruction,
    playlist::Playlist,
};

use super::menu::{Menu, MenuState};
use ratatui::{
    prelude::*,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState},
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter, VariantArray};

#[derive(Display, EnumIter, VariantArray, EnumCount)]
enum PlaylistOptions {
    Play,
}

impl<'a> Into<ListItem<'a>> for PlaylistOptions {
    fn into(self) -> ListItem<'a> {
        ListItem::from(format!("{}", self))
    }
}

#[derive(Debug)]
pub struct PlaylistMenu {
    state: ListState,
    playlist: Playlist,
}

impl Display for PlaylistMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.playlist.title.as_str())
    }
}

impl Menu for PlaylistMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        PlaylistOptions::COUNT
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn enter(&mut self) -> crate::AppResult<crate::app_actions::AppAction> {
        Ok(AppAction::StateAction(Box::new(PlayPlaylist::new(
            self.playlist.clone(),
        ))))
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        let area = area.inner(Margin {
            horizontal: 2,
            vertical: 2,
        });

        ratatui::widgets::Clear::default().render(area, buf);

        // todo -> make smaller hehe

        // Layout::new(direction, constraints)

        // paragraph of data

        // List of actions

        let list = List::new(PlaylistOptions::iter())
            .highlight_symbol(">")
            .highlight_spacing(if focused {
                HighlightSpacing::Always
            } else {
                HighlightSpacing::Never
            })
            .yellow()
            .block(Block::bordered());
        StatefulWidget::render(list, area.clone(), buf, &mut self.state);

        Ok(area)
    }
}

impl PlaylistMenu {
    pub fn new(playlist: Playlist) -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self { state, playlist }
    }
}
