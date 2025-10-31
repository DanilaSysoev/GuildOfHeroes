pub mod components;
mod screens;

use bracket_lib::{
    color::RGBA,
    prelude::{BTerm, BTermBuilder, GameState, main_loop},
};

const GLOBAL_MAP_CONSOLE_INDEX: usize = 0;
const MAIN_MENU_CONSOLE_INDEX: usize = 1;

use crate::{
    config::{GameConfig, load_config},
    core::screens::{GlobalMapScreen, Screen},
    errors::GameUiError,
};
pub struct Game {
    screen: Option<Box<dyn Screen>>,
}

impl Game {
    pub fn new(config: GameConfig) -> Result<Self, GameUiError> {
        Ok(Game { screen: Some(Box::new(GlobalMapScreen::new(&config)?)) })
    }

    pub fn run() -> Result<(), GameUiError> {
        let config = load_config()?;
        let context = Self::build_context(&config)?;

        main_loop(context, Game::new(config)?)?;
        Ok(())
    }

    pub fn switch_screen(&mut self, screen: Box<dyn Screen>) {
        self.screen = Some(screen);
    }

    fn build_context(config: &GameConfig) -> Result<BTerm, GameUiError> {
        let context = BTermBuilder::new()
            .with_title("Guild of Heroes")
            .with_resource_path("resources")
            .with_font(
                config.surface_font.path.clone(),
                config.surface_font.width,
                config.surface_font.height,
            )
            .with_font(
                config.text_font.path.clone(),
                config.text_font.width,
                config.text_font.height,
            )
            .with_tile_dimensions(
                config.surface_font.width,
                config.surface_font.height,
            )
            .with_dimensions(config.camera.width, config.camera.height)
            .with_simple_console(
                config.camera.width,
                config.camera.height,
                config.surface_font.path.clone(),
            )
            .with_simple_console(
                config.camera.width,
                config.camera.height,
                config.text_font.path.clone(),
            )
            .build()?;

        Ok(context)
    }

    fn clear_consoles(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAIN_MENU_CONSOLE_INDEX);
        ctx.cls_bg(RGBA::from_u8(0, 0, 0, 0));

        ctx.set_active_console(GLOBAL_MAP_CONSOLE_INDEX);
        ctx.cls_bg(RGBA::from_u8(0, 0, 0, 0));
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.clear_consoles(ctx);
        if let Some(mut screen) = self.screen.take() {
            screen.tick(ctx, self);
            self.screen = Some(screen);
        }
    }
}
