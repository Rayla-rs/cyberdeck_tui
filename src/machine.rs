use std::io::Cursor;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{AppResult, app::Services, main_menu::MainMenu, menu::Menu};

pub enum Instruction {
    Continue,
    Push(Box<dyn Menu>),
    DropPush(Box<dyn Menu>),
    Next,
}
pub struct Machine {
    stack: Vec<Box<dyn Menu>>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: vec![Box::new(MainMenu::new())],
        }
    }

    pub fn enter(&mut self) -> AppResult<()> {
        if let Some(menu) = self.stack.last_mut() {
            let inst = menu.enter()?;
            self.handle_instruction(inst);
        }
        Ok(())
    }
    pub fn up(&mut self) {
        if let Some(menu) = self.stack.last_mut() {
            menu.up();
        }
    }
    pub fn down(&mut self) {
        if let Some(menu) = self.stack.last_mut() {
            menu.down();
        }
    }

    pub fn tick(&mut self, services: &mut Services) -> AppResult<()> {
        if let Some(menu) = self.stack.last_mut() {
            let inst = menu.tick(services)?;
            self.handle_instruction(inst);
        }
        Ok(())
    }

    pub fn handle_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Continue => {}
            Instruction::Next => {
                let _ = self.stack.pop();
            }
            Instruction::Push(menu) => {
                self.stack.push(menu);
            }
            Instruction::DropPush(menu) => {
                let _ = self.stack.pop();
                self.stack.push(menu);
            }
        }
    }

    /// Render menus from old to new
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) -> AppResult<()> {
        let mut area = area;
        Ok(for menu in self.stack.iter_mut() {
            area = menu.render(area, buf)?
        })
    }

    pub fn is_running(&self) -> bool {
        !self.stack.is_empty()
    }
}
