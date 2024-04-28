use {
    super::Tui,
    crate::{args::Args, TResult},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    ratatui::{prelude::*, widgets::Paragraph},
    std::{
        error::Error,
        io::{self, stdout, Stdout},
        time::{Duration, Instant},
    },
};

#[derive(Debug)]
pub struct Menu {
    counter: u8,
    exit: bool,
}

impl Menu {
    pub fn run(&mut self, terminal: &mut Tui) -> TResult<()> {
        // main loop
        // encapsulate this later
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;

            // let now = Instant::now();
            // while event::poll(Duration::ZERO)? {
            //     self.handle_events()?;
            // }
            // while now.elapsed() < Duration::from_secs(1 / 60) {}
            // figure out best way to handle this
            if event::poll(Duration::from_secs(1 / 60))? {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Hello world! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    }

    fn handle_events(&mut self) -> TResult<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
            }
        }

        Ok(())
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            counter: 0,
            exit: false,
        }
    }
}
