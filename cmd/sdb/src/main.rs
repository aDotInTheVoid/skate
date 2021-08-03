use std::cmp::max;
use std::io::{ErrorKind, Write};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders};

mod docs;

fn named_block(name: &str) -> Block {
    Block::default().title(name).borders(Borders::ALL)
}

macro_rules! dbg {
    () => {
        dump(&std::format!("[{}:{}]\n", std::file!(), std::line!()))
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                dump(&std::format!("[{}:{}] {} = {:#?}\n",
                    std::file!(), std::line!(), std::stringify!($val), &tmp));
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

fn dump(c: &str) {
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("sdb.log")
        .unwrap()
        .write_all(c.as_bytes())
        .unwrap();
}

fn grid(x: u16, y: u16, r: Rect) -> Vec<Vec<Rect>> {
    let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100 / x); x.into()])
        .split(r);

    rows.iter()
        .map(|r| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100 / y); y.into()])
                .split(*r)
        })
        .collect()
}

fn main() -> eyre::Result<()> {
    // Copied From `ref/tui-rs/examples/crossterm_demo.rs`

    // Remove log file, ignoreing if it doesn't exist
    if let Err(ioerr) = std::fs::remove_file("sdb.log") {
        if ioerr.kind() != ErrorKind::NotFound {
            return Err(ioerr.into());
        }
    }

    // Set up terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::terminal::Terminal::new(backend)?;

    terminal.clear()?;

    let mut x = 1;
    let mut y = 1;

    loop {
        terminal.draw(|f| {
            let chunks = grid(x, y, f.size());
            dbg!(&chunks, f.size());
            for rx in 0..x {
                for ry in 0..y {
                    f.render_widget(
                        named_block(&format!("{}-{}", rx, ry)),
                        chunks[usize::from(rx)][usize::from(ry)],
                    );
                }
            }
        })?;

        let event = crossterm::event::read()?;
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event
        {
            match c {
                'q' => break,
                'd' => x = x.saturating_add(1),
                'a' => x = max(1, x.saturating_sub(1)),
                'w' => y = y.saturating_add(1),
                's' => y = max(1, y.saturating_sub(1)),
                _ => {}
            }
        }
    }

    // Restore Terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
