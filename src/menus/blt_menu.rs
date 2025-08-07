use std::fmt::Display;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget};

use crate::blt_client::{Device, DeviceMenu};
use crate::machine::Instruction;

use super::menu::{Menu, MenuState};

#[derive(Debug)]
pub struct BltMenu {
    state: TableState,
    devices: Vec<Device>,
}

impl Display for BltMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("blt_menu")
    }
}

impl Menu for BltMenu {
    fn get_state(&mut self) -> &mut dyn MenuState {
        &mut self.state
    }

    fn get_len(&self) -> usize {
        self.devices.len()
    }

    fn enter(&mut self) -> crate::AppResult<crate::app_actions::AppAction> {
        Ok(Instruction::Push(Box::new(DeviceMenu::new(
            self.devices
                .get(self.state.selected().unwrap())
                .unwrap()
                .clone(),
        )))
        .into())
    }

    fn get_quick_actions(&self) -> Vec<crate::app_actions::AppAction> {
        vec![Instruction::Pop.into()]
    }

    fn tick(&mut self, app_state: &mut crate::app::AppState) -> crate::AppResult<Instruction> {
        self.devices = app_state.devices.values().cloned().collect();
        Ok(Instruction::Continue)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> crate::AppResult<Rect> {
        ratatui::widgets::Clear::default().render(area, buf);

        let header = ["Alias", "Address", "Paired", "Trusted"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let table = Table::new(
            self.devices.iter(),
            [
                Constraint::Min(5),
                Constraint::Length(17),
                Constraint::Length(6),
                Constraint::Length(7),
            ],
        )
        .header(header)
        .highlight_symbol(">")
        .highlight_spacing(if focused {
            HighlightSpacing::Always
        } else {
            HighlightSpacing::Never
        });

        StatefulWidget::render(table, area, buf, &mut self.state);

        Ok(area)
    }
}

impl BltMenu {
    pub fn new() -> Self {
        let mut state = TableState::default();
        state.select_first();
        Self {
            state,
            devices: vec![],
        }
    }
}
