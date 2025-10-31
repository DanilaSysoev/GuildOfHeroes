use bracket_lib::prelude::{BTerm, VirtualKeyCode};

use crate::{core::Game, errors::GameUiError, widgets::geometry::Widget};

pub trait MenuAction {
    fn run(&self, ctx: &mut BTerm, game: &mut Game)
    -> Result<(), GameUiError>;
}

pub struct NullAction;

impl MenuAction for NullAction {
    fn run(&self, _: &mut BTerm, _: &mut Game) -> Result<(), GameUiError> {
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
    items: Vec<(VirtualKeyCode, MenuItem)>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { items: Vec::new() }
    }

    pub fn with_item(
        mut self,
        key: VirtualKeyCode,
        item: MenuItem,
    ) -> Result<Self, GameUiError> {
        if self.items.iter().any(|pair| pair.0 == key) {
            return Err(GameUiError::Configuration {
                what: "Error: try bild menu with repeated key".to_string(),
            });
        }

        self.items.push((key, item));
        Ok(self)
    }

    pub fn select(
        &self,
        index: usize,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError> {
        if let Some(item) = self.items.get(index) {
            return item.1.select(ctx, game);
        }

        Ok(())
    }

    pub fn items(&self) -> &[(VirtualKeyCode, MenuItem)] {
        &self.items
    }

    pub fn select_item_if_user_requested(
        &self,
        ctx: &mut BTerm,
        game: &mut Game,
    ) {
        if let Some(key) = ctx.key
            && let Some(index) =
                self.items.iter().position(|pair| pair.0 == key)
        {
            self.select(index, ctx, game).unwrap();
        }
    }
}

impl Widget for Menu {
    #[rustfmt::skip]
    fn width(&self) -> u32 {
        self
            .items
            .iter()
            .map(|item| item.1.width())
            .max()
            .unwrap_or(0)
    }

    fn height(&self) -> u32 {
        self.items.len() as u32
    }
}
