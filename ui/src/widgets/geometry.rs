pub struct WidgetPosition {
    line: i32,
    column: i32,
}

impl WidgetPosition {
    pub fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn column(&self) -> i32 {
        self.column
    }
}

pub trait Widget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
