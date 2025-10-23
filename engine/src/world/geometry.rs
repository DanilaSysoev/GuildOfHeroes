#[derive(Clone, Copy)]
pub struct TilePos {
    line: i32,
    column: i32
}

impl TilePos {
    pub fn new(line: i32, column: i32) -> Self {
        TilePos {
            line: line,
            column: column
        }
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn column(&self) -> i32 {
        self.column
    }
}
