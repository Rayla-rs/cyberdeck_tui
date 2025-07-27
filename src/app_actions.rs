use std::fmt::Display;

use ratatui::widgets::ListItem;

use crate::{app::AppState, machine::Instruction};

pub trait StateAction: Display {
    fn mutate_state(self, state: &mut AppState);
}

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
