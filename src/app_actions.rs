use std::fmt::{Debug, Display};

use ratatui::widgets::ListItem;

use crate::{app::AppState, machine::Instruction, playlist::Playlist};

pub trait StateAction: Debug + Display {
    fn mutate_state(self: Box<Self>, state: &mut AppState);
}

#[derive(Debug)]
pub struct PlayPlaylist {
    playlist: Playlist,
}

impl Display for PlayPlaylist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Play({})", self.playlist))
    }
}

impl StateAction for Box<PlayPlaylist> {
    fn mutate_state(self: Box<Self>, state: &mut AppState) {
        state.player.play_playlist(self.playlist);
    }
}

#[derive(Debug)]
pub enum AppAction {
    MachineAction(Instruction),
    StateAction(Box<dyn StateAction>),
}

impl From<Instruction> for AppAction {
    fn from(value: Instruction) -> Self {
        Self::MachineAction(value)
    }
}

impl From<Box<dyn StateAction>> for AppAction {
    fn from(value: Box<dyn StateAction>) -> Self {
        Self::StateAction(value)
    }
}

impl Display for AppAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MachineAction(instruction) => f.write_fmt(format_args!("{}", instruction)),
            Self::StateAction(state_action) => f.write_fmt(format_args!("{}", state_action)),
        }
    }
}

impl<'a> Into<ListItem<'a>> for &AppAction {
    fn into(self) -> ListItem<'a> {
        ListItem::new(format!("{self}"))
    }
}
