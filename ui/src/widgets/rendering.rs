use bracket_lib::prelude::BTerm;

use crate::{
    config::MenuRenderedConfig,
    widgets::{geometry::WidgetPosition, menu::Menu},
};

struct MenuRenderer {
    position: WidgetPosition,
}

impl MenuRenderer {
    pub fn new(line: i32, column: i32) -> Self {
        Self { position: WidgetPosition::new(line, column) }
    }

    pub fn render(
        &self,
        menu: &Menu,
        ctx: &mut BTerm,
        config: &MenuRenderedConfig,
    ) {
        todo!();
    }
}
