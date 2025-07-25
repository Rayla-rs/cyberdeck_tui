use ratatui::{buffer::Buffer, layout::Rect};

use crate::{AppResult, app::Services, machine::Instruction};

pub trait Menu {
    fn up(&mut self) {}
    fn down(&mut self) {}
    fn enter(&mut self) -> AppResult<Instruction> {
        Ok(Instruction::Continue)
    }
    fn tick(&mut self, _services: &mut Services) -> AppResult<Instruction> {
        Ok(Instruction::Continue)
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer) -> AppResult<Rect>;
}
