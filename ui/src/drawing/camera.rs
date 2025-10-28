pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Camera {
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        Self { top: 0, left: 0, width, height }
    }

    pub fn with_top(mut self, top: i32) -> Self {
        self.top = top;
        self
    }

    pub fn with_left(mut self, left: i32) -> Self {
        self.left = left;
        self
    }

    pub fn with_top_left(self, top: i32, left: i32) -> Self {
        self.with_top(top).with_left(left)
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.left + self.width - 1
    }

    pub fn bottom(&self) -> i32 {
        self.top + self.height - 1
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn contains(&self, line: i32, column: i32) -> bool {
        line >= self.top()
            && line <= self.bottom()
            && column >= self.left()
            && column <= self.right()
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.top -= 1,
            Direction::Down => self.top += 1,
            Direction::Left => self.left -= 1,
            Direction::Right => self.left += 1,
        }
    }
}
