use {
    crate::{args::Args, TResult},
    core::time,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    menu::Menu,
    ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        widgets::Paragraph,
    },
    std::{
        error::Error,
        io::{self, stdout, Stdout},
        time::Duration,
    },
};

pub mod menu;
pub mod settings;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn run_tui(args: Args) -> TResult<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let result = Menu::default().run(&mut terminal);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    result
}
