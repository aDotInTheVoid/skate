use std::convert::TryFrom;

use tui::layout::Rect;

enum Size {
    Fr(u16),
    Px(u16),
}

/*
 * +----+----+----+----+----+----+----+----+
 * |    |    |    |    |    |    |    |    |  <- Row
 * +----+----+----+----+----+----+----+----+
 * |    |    |    |    |    |    |    |    |
 * +----+----+----+----+----+----+----+----+
 * |    |    |    |    |    |    |    |    |
 * +----+----+----+----+----+----+----+----+
 * |    |    |    |    |    |    |    |    |
 * +----+----+----+----+----+----+----+----+
 *    ^
 *    |
 *    Column
 */

#[derive(Clone, Copy)]
pub struct Grid {
    rows: u16,
    cols: u16,
}

impl Grid {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols }
    }

    pub fn size(self, size: Rect) -> SizedGrid {
        // TODO: Allow non even lines, alla CSS Grid
        // TODO: Cache layouts
        let xpos = poses(size.x, size.width, self.cols);
        let ypos = poses(size.y, size.height, self.rows);

        let mut cells = Vec::new();

        // TODO: Use array_windows when it's stable
        for [xstart, xstop] in xpos.windows(2).map(|x| <[u16; 2]>::try_from(x).unwrap()) {
            for [ystart, ystop] in ypos.windows(2).map(|x| <[u16; 2]>::try_from(x).unwrap()) {
                cells.push(Rect {
                    x: xstart,
                    y: ystart,
                    width: xstop - xstart,
                    height: ystop - ystart,
                })
            }
        }

        SizedGrid { cells }
    }
}

// Takes the initalial value, the total lenght, and the number to divide into, and returns
// a list of positions
fn poses(init: u16, len: u16, num: u16) -> Vec<u16> {
    let thin_width = len / num;
    let thick_width = thin_width + 1;

    let n_thick = len % num;
    let n_thin = num - n_thick;

    let mut pos = init;
    let mut out = Vec::with_capacity((num + 1).into());
    out.push(pos);
    for _ in 0..n_thick {
        pos += thick_width;
        out.push(pos);
    }
    for _ in 0..n_thin {
        pos += thin_width;
        out.push(pos);
    }

    out
}

pub struct SizedGrid {
    pub cells: Vec<Rect>,
}
