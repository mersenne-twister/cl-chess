use {
    crate::{Handle, Render, Screen, Terminal},
    crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent},
    ratatui::{layout::Position, prelude::*, widgets::Paragraph},
    std::{cell::RefCell, rc::Rc},
    termchess_common::TResult,
    termchess_engine::board::{
        print::{BoardOptions, Size},
        Board,
    },
};

pub mod board;
pub mod console;

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
                // size: Size::UnicodeArt,
                size: Size::BlockArt,
                ..Default::default()
            },
        }
    }

    fn title_widget(&self) -> Paragraph {
        Paragraph::new(Text::from("t e r m c h e s s".fg(Color::Indexed(7)).bold())).centered()
    }
}

// #[allow(unused_variables)] // I forget why
impl Screen for Game {
    fn terminal(&self) -> &Rc<RefCell<Terminal>> {
        &self.terminal
    }

    fn mouse_pos(&mut self) -> &mut Position {
        &mut self.mouse_pos
    }

    fn exit(&self) -> bool {
        self.exit
    }
}

impl Render for Game {
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
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(self.board_options.height() as u16),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(layout[1]);
        self.board_pos = board_layout[3].as_position();

        let console_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.board_options.width() as u16 - 10),
                Constraint::Fill(1),
            ])
            .split(board_layout[5]);

        frame.render_widget(self.title_widget(), board_layout[1]);
        frame.render_widget(self.board_widget()?, board_layout[3]);
        frame.render_widget(self.console_widget(), console_layout[1]);

        Ok(())
    }

    fn background_color(&self) -> Color {
        Color::Indexed(232)
    }
}

impl Handle for Game {
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

        Ok(())
    }

    fn handle_misc(&mut self, event: Event) -> TResult<()> {
        Ok(())
    }
}
