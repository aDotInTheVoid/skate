use std::io::{ErrorKind, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders};

use crate::grid::Direction;

mod docs;
mod grid;
mod term;

fn named_block(name: &str) -> Block {
    Block::default().title(name).borders(Borders::ALL)
}

#[allow(unused_macros)] // For debugging
#[macro_export]
macro_rules! dbg {
    () => {
        dump(&std::format!("[{}:{}]\n", std::file!(), std::line!()))
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::dump(&std::format!("[{}:{}] {} = {:?}\n",
                    std::file!(), std::line!(), std::stringify!($val), &tmp));
                //  dump(&std::format!("[{}:{}] {} = {}\n",
                // std::file!(), std::line!(), std::stringify!($val), debug2::pprint(&tmp)));
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

// For debugging
#[allow(dead_code)]
fn dump(c: &str) {
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("sdb.log")
        .unwrap()
        .write_all(c.as_bytes())
        .unwrap();
}

fn main() -> eyre::Result<()> {
    // Remove log file, ignoreing if it doesn't exist
    if let Err(ioerr) = std::fs::remove_file("sdb.log") {
        if ioerr.kind() != ErrorKind::NotFound {
            return Err(ioerr.into());
        }
    }

    term::install_panic();

    let mut terminal = term::Terminal::new()?;

    let mut grid = grid::Grid::new(4, 4);

    let b_main = grid.claim(grid::Block {
        row_start: 0,
        row_stop: 2,
        col_start: 0,
        col_stop: 2,
    });
    let mut sel_id = b_main;

    let b_corner = grid.claim(grid::Block {
        row_start: 2,
        row_stop: 3,
        col_start: 2,
        col_stop: 3,
    });
    let b_bottom = grid.claim(grid::Block {
        row_start: 3,
        row_stop: 4,
        col_start: 0,
        col_stop: 4,
    });
    let b_right = grid.claim(grid::Block {
        row_start: 0,
        row_stop: 3,
        col_start: 3,
        col_stop: 4,
    });
    let b_inner_right = grid.claim(grid::Block {
        row_start: 0,
        row_stop: 2,
        col_start: 2,
        col_stop: 3,
    });
    let b_inner_bottom = grid.claim(grid::Block {
        row_start: 2,
        row_stop: 3,
        col_start: 0,
        col_stop: 2,
    });

    let mut hitboxes = Vec::new();

    loop {
        terminal.draw(|f| {
            hitboxes.clear();
            let sg = grid.size(f.size());

            for (i, bid) in [
                b_main,
                b_corner,
                b_bottom,
                b_right,
                b_inner_right,
                b_inner_bottom,
            ]
            .iter()
            .enumerate()
            {
                let r = sg.size_of(*bid);
                let name = i.to_string();
                let mut block = named_block(&name);
                if sel_id == *bid {
                    block = block.style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Yellow),
                    );
                }
                hitboxes.push((*bid, r));
                f.render_widget(block, r);
            }
        })?;

        let event = crossterm::event::read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => match c {
                'q' => break,
                // 'd' => sel_x = min(2, sel_x.saturating_add(1)),
                // 'a' => sel_x = min(2, sel_x.saturating_sub(1)),
                'w' => sel_id = grid.go(sel_id, Direction::Up).unwrap_or(sel_id),
                'a' => sel_id = grid.go(sel_id, Direction::Left).unwrap_or(sel_id),
                's' => sel_id = grid.go(sel_id, Direction::Down).unwrap_or(sel_id),
                'd' => sel_id = grid.go(sel_id, Direction::Right).unwrap_or(sel_id),
                // 's' => sel_y = min(2, sel_y.saturating_add(1)),
                _ => {}
            },
            Event::Mouse(MouseEvent {
                row: mouse_y,
                column: mouse_x,
                kind: MouseEventKind::Down(MouseButton::Left),
                ..
            }) => {
                for (id, rect) in hitboxes.iter() {
                    if contains(mouse_x, mouse_y, *rect) {
                        sel_id = *id;
                    }
                }
            }
            _ => (),
        }
    }

    terminal.close()?;

    Ok(())
}

fn contains(x: u16, y: u16, rect: Rect) -> bool {
    (rect.x..=rect.x + rect.width).contains(&x) && (rect.y..=rect.y + rect.height).contains(&y)
}
