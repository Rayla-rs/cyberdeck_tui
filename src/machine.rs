use std::fmt::Display;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{AppResult, app::AppState, menus::Menu};

#[derive(Debug)]
pub enum Instruction {
    Continue,
    Push(Box<dyn Menu>),
    PopPush(Box<dyn Menu>),
    Pop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Continue => String::from("continue"),
                Self::Push(menu) => format!("push({menu})"),
                Self::PopPush(menu) => format!("pop_push({menu})"),
                Self::Pop => String::from("pop"),
            }
            .as_str(),
        )
    }
}

pub struct Machine {
    stack: Vec<Box<dyn Menu>>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.stack
                .iter()
                .map(|menu| format!("{}", menu))
                .collect::<Vec<_>>()
                .join("/")
                .as_str(),
        )
    }
}

impl Machine {
    // pub fn new() -> Self {
    //     Self {
    //         stack: vec![Box::new(MainMenu::new())],
    //     }
    // }

    pub fn last_mut(&mut self) -> &mut Box<dyn Menu> {
        self.stack.last_mut().unwrap()
    }

    pub fn last(&self) -> &Box<dyn Menu> {
        self.stack.last().unwrap()
    }

    pub fn tick(&mut self, app_state: &mut AppState) -> AppResult<()> {
        if let Some(menu) = self.stack.last_mut() {
            let inst = menu.tick(app_state)?;
            self.handle_instruction(inst);
        }
        Ok(())
    }

    pub fn handle_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Continue => {}
            Instruction::Pop => {
                drop(self.stack.pop());
            }
            Instruction::Push(menu) => {
                self.stack.push(menu);
            }
            Instruction::PopPush(menu) => {
                drop(self.stack.pop());
                self.stack.push(menu);
            }
        }
    }

    /// Render menus from old to new
    pub fn render(&mut self, area: Rect, buf: &mut Buffer, focused: bool) -> AppResult<()> {
        let mut area = area;
        let mut iter = self.stack.iter_mut().peekable();
        Ok(while let Some(menu) = iter.next() {
            area = menu.render(
                area,
                buf,
                if iter.peek().is_none() {
                    focused
                } else {
                    false
                },
            )?
        })
    }

    pub fn is_running(&self) -> bool {
        !self.stack.is_empty()
    }
}
