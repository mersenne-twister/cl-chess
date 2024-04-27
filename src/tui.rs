use {
    crate::args::Args,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        widgets::Paragraph,
    },
    std::{error::Error, io::stdout},
};

pub fn run_tui(args: Args) -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(())
}
