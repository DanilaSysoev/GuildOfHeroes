use bracket_lib::{
    color::{BLACK, RGB, WHITE},
    prelude::BTerm,
};

use crate::{
    config::MenuRenderedConfig,
    widgets::{geometry::Widget, menu::Menu},
};

pub struct MenuRenderer {
    line: i32,
    column: i32,
}

impl MenuRenderer {
    pub fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }

    pub fn render(
        &self,
        menu: &Menu,
        ctx: &mut BTerm,
        config: &MenuRenderedConfig,
    ) {
        ctx.draw_box(
            self.column,
            self.line,
            menu.width() + config.left_border + config.right_border + 1,
            menu.height() + config.top_border + config.bottom_border + 1,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );

        for (index, item) in menu.items().iter().enumerate() {
            ctx.print(
                self.column + 1 + config.left_border as i32,
                self.line + 1 + config.top_border as i32 + index as i32,
                item.text(),
            );
        }
    }
}
