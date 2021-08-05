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
    // rows: Vec<Size>,
    // cols: Vec<Size>,
}

impl Grid {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols }
    }

    pub fn size(self, size: Rect) -> SizedGrid {
        let cell_width = size.width / self.cols;
        let wide_cells = size.width % self.cols;
        let thin_cells = self.cols - wide_cells;

        let mut cells = Vec::with_capacity((self.rows * self.cols).into());

        let mut xpos = size.x;
        // let mut xposs = Vec::with_capacity((self.rows + 1).into());

        for _ in 0..wide_cells {
            cells.push(Rect {
                x: xpos,
                y: size.y,
                width: cell_width + 1,
                height: size.height,
            });
            xpos += cell_width + 1;
        }
        for _ in 0..thin_cells {
            cells.push(Rect {
                x: xpos,
                y: size.y,
                width: cell_width,
                height: size.height,
            });
            xpos += cell_width;
        }

        SizedGrid { cells }
    }
}

pub struct SizedGrid {
    pub cells: Vec<Rect>,
}
