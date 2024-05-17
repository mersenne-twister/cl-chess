use {
    crate::{Screen, Terminal},
    crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent},
    ratatui::{layout::Position, prelude::*, widgets::Paragraph},
    std::{cell::RefCell, rc::Rc},
    termchess_common::TResult,
    termchess_engine::board::{
        print::{BoardOptions, Size},
        Board, Color,
    },
};

pub mod board;

pub struct Game {
    board: Board,
    exit: bool,
    mouse_pos: Position,
    board_pos: Position,
    terminal: Rc<RefCell<Terminal>>,
    board_options: BoardOptions,
}

impl Game {
    pub fn new(terminal: Rc<RefCell<Terminal>>) -> Self {
        Self {
            board: Board::new(),
            exit: false,
            mouse_pos: Position::default(),
            board_pos: Position::default(),
            terminal,
            board_options: BoardOptions {
                // size: Size::Letters {
                //     different_symbols: false,
                // },
                size: Size::UnicodeArt,
                // size: Size::BlockArt,
                ..Default::default()
            },
        }
    }
}

#[allow(unused_variables)]
impl Screen for Game {
    fn render_frame(&mut self, frame: &mut Frame) -> TResult<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.board_options.width() as u16),
                Constraint::Fill(1),
            ])
            .split(frame.size());

        let board_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.board_options.height() as u16),
                Constraint::Fill(1),
            ])
            .split(layout[1]);

        frame.render_widget(self.board_widget()?, board_layout[1]);

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> TResult<()> {
        if key.modifiers == KeyModifiers::NONE {
            match key.code {
                _ => (),
            }
        } else if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                crossterm::event::KeyCode::Char('c') => self.exit = true,
                _ => (),
            }
        }

        Ok(())
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> TResult<()> {
        // TODO: add mut mouse_pos getter, handle movement in there,
        // then separate functions for clicks?
        self.mouse_pos = Position::new(mouse.column, mouse.row);

        // TODO
        Ok(())
    }

    fn handle_misc(&mut self, event: Event) -> TResult<()> {
        Ok(())
    }

    fn terminal(&self) -> &Rc<RefCell<Terminal>> {
        &self.terminal
    }

    fn exit(&self) -> bool {
        self.exit
    }
}
