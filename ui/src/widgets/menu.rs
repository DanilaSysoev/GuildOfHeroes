use bracket_lib::prelude::BTerm;

use crate::{
    core::{Game, components::GameEntity},
    errors::GameUiError,
    widgets::geometry::Widget,
};

pub trait MenuAction {
    fn run(&self, ctx: &mut BTerm, game: &mut Game)
    -> Result<(), GameUiError>;
}

pub struct NullAction;

impl MenuAction for NullAction {
    fn run(&self, _: &mut BTerm, game: &mut Game) -> Result<(), GameUiError> {
        Ok(())
    }
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

    pub fn select(
        &self,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError> {
        self.action.run(ctx, game)
    }
}

impl Widget for MenuItem {
    fn width(&self) -> u32 {
        self.text.chars().count() as u32
    }
    fn height(&self) -> u32 {
        1
    }
}

#[derive(Default)]
pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { items: Vec::new() }
    }

    pub fn with_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn select(
        &self,
        index: usize,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError> {
        if let Some(item) = self.items.get(index) {
            return item.select(ctx, game);
        }

        Ok(())
    }

    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }
}

impl Widget for Menu {
    #[rustfmt::skip]
    fn width(&self) -> u32 {
        self
            .items
            .iter()
            .map(|item| item.width())
            .max()
            .unwrap_or(0)
    }

    fn height(&self) -> u32 {
        self.items.len() as u32
    }
}

impl GameEntity for Menu {
    fn update(&mut self, ctx: &mut BTerm) {
        todo!()
    }
}
