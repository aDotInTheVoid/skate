use std::convert::TryFrom;

use slotmap::SlotMap;
use tui::layout::Rect;

// TODO: Consider a NonZeroU16 for Niche
slotmap::new_key_type! { pub struct BlockId; }

#[derive(Debug, Clone, Copy)]
/**
```text
   0    1    2    3    4    5    6    7    8
 0 +----+----+----+----+----+----+----+----+
   |    |    |    |    |    |    |    |    |  <- Row
 1 +----+----XXXXXXXXXXXXXXXX----+----+----+
   |    |    XXXXXXXXXXXXXXXX    |    |    |
 2 +----+----XXXXXXXXXXXXXXXX----+----+----+
   |    |    XXXXXXXXXXXXXXXX    |    |    |
 3 +----+----XXXXXXXXXXXXXXXX----+----+----+
   |    |    |    |    |    |    |    |    |
 4 +----+----+----+----+----+----+----+----+
      ^
      |
      Column
```

```rust
Block {
    row_start: 1,
    row_stop: 3,
    col_start: 2,
    col_stop: 5,
}
```
*/
pub struct Block {
    pub row_start: u16,
    pub row_stop: u16,
    pub col_start: u16,
    pub col_stop: u16,
}

/**
A grid of rows and columns

```rust
Grid {
    rows: 4
    cols: 8
}
```

```text
   0    1    2    3    4    5    6    7    8
 0 +----+----+----+----+----+----+----+----+
   |    |    |    |    |    |    |    |    |  <- Row
 1 +----+----+----+----+----+----+----+----+
   |    |    |    |    |    |    |    |    |
 2 +----+----+----+----+----+----+----+----+
   |    |    |    |    |    |    |    |    |
 3 +----+----+----+----+----+----+----+----+
   |    |    |    |    |    |    |    |    |
 4 +----+----+----+----+----+----+----+----+
      ^
      |
      Column
```
*/
#[derive(Clone)]

pub struct Grid {
    rows: u16,
    cols: u16,
    boxes: Vec<Vec<Option<BlockId>>>,
    ids: SlotMap<BlockId, Block>,
}

impl Grid {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            rows,
            cols,
            boxes: vec![vec![None; rows.into()]; cols.into()],
            ids: SlotMap::with_key(),
        }
    }

    pub fn claim(&mut self, block: Block) -> BlockId {
        let id = self.ids.insert(block);
        for x in block.col_start..block.col_stop {
            for y in block.row_start..block.row_stop {
                // TODO: Enforce Uniqueness of claim
                self.boxes[usize::from(x)][usize::from(y)] = Some(id);
            }
        }
        id
    }

    pub fn size(&self, size: Rect) -> SizedGrid<'_> {
        // TODO: Allow non even lines, alla CSS Grid
        // TODO: Cache layouts
        let xpos = poses(size.x, size.width, self.cols);
        let ypos = poses(size.y, size.height, self.rows);

        // let mut cells = Vec::new();

        // // TODO: Use array_windows when it's stable
        // for [xstart, xstop] in xpos.windows(2).map(|x| <[u16; 2]>::try_from(x).unwrap()) {
        //     for [ystart, ystop] in ypos.windows(2).map(|x| <[u16; 2]>::try_from(x).unwrap()) {
        //         cells.push(Rect {
        //             x: xstart,
        //             y: ystart,
        //             width: xstop - xstart,
        //             height: ystop - ystart,
        //         })
        //     }
        // }

        SizedGrid {
            grid: self,
            xpos,
            ypos,
        }
    }
}

impl SizedGrid<'_> {
    pub fn size_of(&self, block: BlockId) -> Rect {
        let Block {
            row_start,
            row_stop,
            col_start,
            col_stop,
        } = self.grid.ids[block];

        let xstart = self.xpos[usize::from(col_start)];
        let xstop = self.xpos[usize::from(col_stop)];
        let ystart = self.ypos[usize::from(row_start)];
        let ystop = self.ypos[usize::from(row_stop)];

        Rect {
            x: xstart,
            y: ystart,
            width: xstop - xstart,
            height: ystop - ystart,
        }
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

pub struct SizedGrid<'g> {
    xpos: Vec<u16>,
    ypos: Vec<u16>,
    grid: &'g Grid,
}
