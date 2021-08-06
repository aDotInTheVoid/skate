use std::io::{self, Stdout};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::terminal::CompletedFrame;
use tui::Frame;

// TODO: Buffer stdout
type Backend = CrosstermBackend<Stdout>;

pub struct Terminal {
    inner: tui::Terminal<Backend>,
}

pub fn install_panic() {
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(mut term) = Terminal::new() {
            let _ = term.close();
        }
        old_hook(info);
    }))
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        // Set up terminal
        // Copied From `ref/tui-rs/examples/crossterm_demo.rs`
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = tui::backend::CrosstermBackend::new(stdout);
        let mut terminal = tui::terminal::Terminal::new(backend)?;
        terminal.clear()?;

        Ok(Self { inner: terminal })
    }

    pub fn close(&mut self) -> io::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        execute!(
            self.inner.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.inner.show_cursor()?;
        Ok(())
    }

    // Forwarding to [`tui::terminal::Terminal::draw`]
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<Backend>),
    {
        self.inner.draw(f)
    }
}
