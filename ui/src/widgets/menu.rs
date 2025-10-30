use bracket_lib::prelude::BTerm;

use crate::core::GameEntity;

pub trait MenuAction {
    fn run(&self, ctx: &mut BTerm);
}

pub struct NullAction;

impl MenuAction for NullAction {
    fn run(&self, _: &mut BTerm) {}
}

pub struct MenuItem {
    text: String,
    action: Box<dyn MenuAction>,
}

impl MenuItem {
    pub fn new(text: &str) -> Self {
        MenuItem { text: text.to_string(), action: Box::new(NullAction) }
    }

    pub fn with_action(mut self, action: Box<dyn MenuAction>) -> Self {
        self.action = action;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn select(&self, ctx: &mut BTerm) {
        self.action.run(ctx);
    }

    pub fn width(&self) -> u32 {
        self.text.chars().count() as u32
    }
}

#[derive(Default)]
pub struct Menu {
    console_index: usize,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(console_index: usize) -> Self {
        Menu { console_index, items: Vec::new() }
    }

    pub fn with_item(&mut self, item: MenuItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn width(&self) -> u32 {
        self.items.iter().map(|item| item.width()).max().unwrap_or(0)
    }

    pub fn height(&self) -> u32 {
        self.items.len() as u32
    }
}

impl GameEntity for Menu {
    fn update(&mut self, ctx: &mut BTerm) {
        todo!()
    }
}
