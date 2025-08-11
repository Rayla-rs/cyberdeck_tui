use std::fmt::{Debug, Display};

use ratatui::widgets::ListItem;
use serde::de::value;

use crate::{app::AppState, blt_client::Device, machine::Instruction, playlist::Playlist};

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

impl StateAction for PlayPlaylist {
    fn mutate_state(self: Box<Self>, state: &mut AppState) {
        state.player.queue_playlist(self.playlist);
        state.player.play();
    }
}

impl PlayPlaylist {
    pub fn new(playlist: Playlist) -> Self {
        Self { playlist }
    }
}

#[derive(Debug)]
pub struct ClearLog;

impl Display for ClearLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClearLog")
    }
}

impl StateAction for ClearLog {
    fn mutate_state(self: Box<Self>, _state: &mut AppState) {
        todo!()
    }
}

#[derive(Debug)]
pub struct PairDevice {
    device: Device,
}

impl Display for PairDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("pair({:?})", self.device))
    }
}

impl AppOnce for PairDevice {
    fn once(self: Box<Self>) {
        tokio::spawn(async move {
            let _ = self.device.bt_device.connect().await;
            let _ = self.device.bt_device.pair().await;
        });
    }
}

impl PairDevice {
    pub fn new(device: Device) -> Self {
        Self { device }
    }
}

pub trait AppOnce: Display + Debug {
    fn once(self: Box<Self>);
}

#[derive(Debug)]
pub enum AppAction {
    MachineAction(Instruction),
    StateAction(Box<dyn StateAction>),
    Once(Box<dyn AppOnce>),
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
            Self::Once(once) => f.write_fmt(format_args!("{}", once)),
        }
    }
}

impl<'a> Into<ListItem<'a>> for &AppAction {
    fn into(self) -> ListItem<'a> {
        ListItem::new(format!("{self}"))
    }
}
