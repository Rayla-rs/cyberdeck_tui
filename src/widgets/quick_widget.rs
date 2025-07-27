use std::fmt::Display;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{AppResult, app::AppState, machine::Instruction};

pub trait StateAction: Display {
    fn mutate_state(self, state: &mut AppState);
}

pub enum QuickActions {
    MachineInstruction(Instruction),
    MutateState(Box<dyn StateAction>),
}

impl Display for QuickActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MachineInstruction(instruction) => f.write_fmt(format_args!("{}", instruction)),
            Self::MutateState(state_action) => f.write_fmt(format_args!("{}", state_action)),
        }
    }
}

impl<'a> Into<ListItem<'a>> for &QuickActions {
    fn into(self) -> ListItem<'a> {
        ListItem::new(format!("{self}"))
    }
}

pub struct QuickWidget {
    state: ListState,
    actions: Vec<QuickActions>,
}

impl QuickWidget {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            actions: vec![QuickActions::MachineInstruction(Instruction::Pop)],
        }
    }

    pub fn get_height(&self) -> usize {
        if self.actions.is_empty() {
            0
        } else {
            self.actions.len() + 1
        }
    }

    pub fn enter(&mut self) -> AppResult<()> {
        todo!()
    }
}

impl Widget for &mut QuickWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let list = List::default()
            .block(Block::new().borders(Borders::TOP))
            .items(self.actions.iter())
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
