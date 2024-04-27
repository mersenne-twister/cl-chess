use {
    crate::args::Args, core::time, crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, ratatui::{
        prelude::{CrosstermBackend, Stylize, Terminal},
        widgets::Paragraph,
    }, std::{time::Duration, error::Error, io::stdout}
};

pub fn run_tui(args: Args) -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main loop
    // encapsulate this later
    loop {
        //draw ui
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello world! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
            );
        })?;

        //handle events
        if event::poll(time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                break;
            }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
