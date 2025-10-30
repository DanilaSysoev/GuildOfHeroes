use crate::errors::GameUiError;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Camera {
    top: i32,
    left: i32,
    base_width: u32,
    base_height: u32,
    min_width: u32,
    max_width: u32,
    width: u32,
    height: u32,
    zoom_step: u32,
}

impl Camera {
    pub fn new(
        width: u32,
        min_width: u32,
        max_width: u32,
        height: u32,
        zoom_step: u32,
    ) -> Result<Self, GameUiError> {
        Self {
            top: 0,
            left: 0,
            base_width: width,
            base_height: height,
            width,
            height,
            min_width,
            max_width,
            zoom_step,
        }
        .validate()
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

    pub fn with_zoom_step(mut self, zoom_step: u32) -> Self {
        self.zoom_step = zoom_step;
        self
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.left + self.width as i32 - 1
    }

    pub fn bottom(&self) -> i32 {
        self.top + self.height as i32 - 1
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn line_to_screen(&self, line: i32) -> i32 {
        line - self.top
    }

    pub fn column_to_screen(&self, column: i32) -> i32 {
        column - self.left
    }

    pub fn line_to_world(&self, line: i32) -> i32 {
        line + self.top
    }

    pub fn column_to_world(&self, column: i32) -> i32 {
        column + self.left
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

    pub fn zoom_in(&mut self) {
        self.width -= self.zoom_step;
        if self.width < self.min_width {
            self.width = self.min_width;
        }
        self.recalculate_height();
    }

    pub fn zoom_out(&mut self) {
        self.width += self.zoom_step;
        if self.width >= self.max_width {
            self.width = self.max_width
        }
        self.recalculate_height();
    }

    pub fn zoom_reset(&mut self) {
        self.width = self.base_width;
        self.height = self.base_height;
    }

    fn recalculate_height(&mut self) {
        self.height = ((self.base_height * self.width) as f32
            / self.base_width as f32)
            .ceil() as u32;
    }

    fn validate(self) -> Result<Self, GameUiError> {
        let mut errors = Vec::<&str>::new();
        if self.width < self.min_width || self.width > self.max_width {
            errors.push(
                "Error: Camera width out of bounds [min_width, max_width]",
            );
        }
        if self.min_width > self.max_width {
            errors.push("Error: Camera min_width greater than max_width");
        }
        if self.min_width == 0 {
            errors.push("Error: Camera min_width cannot be zero");
        }
        if self.zoom_step < 1 {
            errors.push("Error: Camera zoom_step cannot be less than 1");
        }

        if errors.is_empty() {
            return Ok(self);
        }

        Err(GameUiError::from(errors.join("\n")))
    }
}
