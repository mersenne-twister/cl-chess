use {
    crate::{Screen, Terminal},
    ratatui::layout::Position,
    std::{cell::RefCell, rc::Rc},
};

pub mod board;

pub struct Game {
    exit: bool,
    mouse_pos: Position,
    terminal: Rc<RefCell<Terminal>>,
}

impl Game {
    pub fn new(terminal: Rc<RefCell<Terminal>>) -> Self {
        Self {
            exit: false,
            mouse_pos: Position::default(),
            terminal,
        }
    }
}

impl Screen for Game {
    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame) {
        todo!()
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> termchess_common::TResult<()> {
        todo!()
    }

    fn handle_mouse(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> termchess_common::TResult<()> {
        todo!()
    }

    fn handle_misc(&mut self, event: crossterm::event::Event) -> termchess_common::TResult<()> {
        todo!()
    }

    fn terminal(&self) -> &Rc<RefCell<Terminal>> {
        &self.terminal
    }

    fn exit(&self) -> bool {
        self.exit
    }
}
