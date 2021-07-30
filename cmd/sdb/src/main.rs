use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufWriter};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

#[cfg(windows)]
type Stdout = io::Stdout;

#[cfg(unix)]
type Stdout = File;

fn main() {
    // Coppied from cursive/backends/crossterm.rs

    crossterm::terminal::enable_raw_mode().unwrap();

    // TODO: Use the stdout we define down there
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture, Hide).unwrap();

    #[cfg(unix)]
    let stdout = RefCell::new(BufWriter::new(File::create("/dev/tty").unwrap()));
    #[cfg(windows)]
    let stdout = RefCell::new(BufWriter::new(io::stdout()));

    let stdout: RefCell<BufWriter<Stdout>> = stdout;

    //
    // Cleanup
    //

    // We have to execute the show cursor command at the `stdout`.
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )
    .expect("Can not disable mouse capture or show cursor.");

    disable_raw_mode().unwrap();
}
