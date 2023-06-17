#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//struct Vec2(i32, i32);
//
//impl From<Direction> for Vec2 {
//    fn from(dir: Direction) -> Self {
//        match dir {
//            Direction::Up => Vec2(-1, 0),
//            Direction::Down => Vec2(1, 0),
//            Direction::Left => Vec2(0, -1),
//            Direction::Right => Vec2(0, 1)
//        };
//    }
//}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Wall,
    Empty,
}

#[derive(Debug)]
pub struct GridCoords {
    pub h: u32,
    pub w: u32,
}

pub struct Grid {
    /// The number of rows or columns
    pub size: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(size: u32) -> Grid {
        Grid {
            size,
            cells: vec![Cell::Wall; (size * size) as usize],
        }
    }

    pub fn get_cell_coords(&self, coord: &GridCoords) -> Option<&Cell> {
        self.cells.get((coord.h * self.size + coord.w) as usize)
    }

    pub fn get_cell(&self, h: u32, w: u32) -> Option<&Cell> {
        self.cells.get((h * self.size + w) as usize)
    }
}
