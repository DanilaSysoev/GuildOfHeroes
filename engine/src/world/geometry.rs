#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TilePos {
    line: i32,
    column: i32,
}

impl TilePos {
    pub fn new(line: i32, column: i32) -> Self {
        TilePos { line, column }
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn column(&self) -> i32 {
        self.column
    }
}
