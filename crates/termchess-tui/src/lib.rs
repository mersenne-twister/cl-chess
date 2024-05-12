use {
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyEventKind,
            MouseEvent,
        },
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    menu::Menu,
    ratatui::prelude::{Terminal as RatatuiTerminal, *},
    std::{
        cell::RefCell,
        io::{stdout, Stdout},
        panic,
        rc::Rc,
        time::{Duration, Instant},
    },
    termchess_common::{Args, TResult},
};

pub mod ascii;
pub mod game;
pub mod menu;
pub mod settings;

pub type Terminal = RatatuiTerminal<CrosstermBackend<Stdout>>;

#[allow(unused_variables)]
pub fn run(args: Args) -> TResult<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    stdout().execute(EnableMouseCapture)?;

    // if this code panics, raw mode etc will still be disabled
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        // ignore err, nothing we can do anyways
        let _ = terminal_normal_mode();
        prev(info);
    }));

    let mut terminal = RatatuiTerminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let result = Menu::new(Rc::new(RefCell::new(terminal))).run();

    terminal_normal_mode()?;

    let _ = panic::take_hook();

    result
}

fn terminal_normal_mode() -> TResult<()> {
    stdout().execute(DisableMouseCapture)?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// if a return is needed for any of these, make them generic over TResult<T>
trait Screen {
    fn run(mut self) -> TResult<()>
    where
        Self: Sized,
    {
        let mut last = Instant::now();
        while !self.exit() {
            if Instant::now() >= last + Duration::from_secs(1 / 60) {
                last = Instant::now();

                // let terminal = self.terminal();
                // terminal.draw(|frame| self.render_frame(frame))?;
                Rc::clone(self.terminal())
                    .borrow_mut()
                    .draw(|frame| self.render_frame(frame).unwrap())?;
            }

            if event::poll(Duration::from_secs(1 / 60))? {
                let event = event::read()?;
                if let Event::Key(event) = event {
                    // stops duplicate events on windows
                    if event.kind == KeyEventKind::Press {
                        self.handle_key(event)?;
                    }
                } else if let Event::Mouse(event) = event {
                    self.handle_mouse(event)?;
                } else {
                    self.handle_misc(event)?;
                }
            }
        }

        Ok(())
    }

    /// function to handle rendering
    /// reccomended to use only for layout, and use getter functions for the widgets
    fn render_frame(&mut self, frame: &mut Frame) -> TResult<()>;

    /// handles all the key input
    fn handle_key(&mut self, key: KeyEvent) -> TResult<()>;

    /// handles the mouse input
    fn handle_mouse(&mut self, mouse: MouseEvent) -> TResult<()>;

    /// handles everything not key or mouse related
    /// (guranteed not to be `KeyEvent` or `MouseEvent`)
    fn handle_misc(&mut self, event: Event) -> TResult<()>;

    ///getter for the terminal field
    /// `Rc<RefCell<T>>` is necessary to be able to call draw on the terminal,
    /// while still mutating self in that method
    fn terminal(&self) -> &Rc<RefCell<Terminal>>;

    /// getter determining when to exit
    fn exit(&self) -> bool;

    // unresolved questions:

    // how should the various widgets work
    // current solution: make them methods/proc funcs if they only have
    // one behavior, make them proper widgets if I want to
    // be able to customize their functionaliby?
    // or just take arguments?

    // for settings, there's a preview that shows your current layout
    // allow defining arbitrary layouts

    // arg_widget takes size so I can have a dynamic size? using something
    // to convert the art on the fly?

    // need to decide how I want starting new screens to work

    // how to get terminal to the new screen
    // just pass it terminal, now that it'll have to be a field

    // no default contructor
}
