pub mod components;
mod states;

use bracket_lib::prelude::{BTerm, BTermBuilder, GameState, main_loop};

use crate::{
    config::{GameConfig, load_config},
    core::states::{GlobalMapScreen, Screen},
    errors::GameUiError,
};
pub struct Game {
    screen: Box<dyn Screen>,
}

impl Game {
    pub fn new(config: GameConfig) -> Result<Self, GameUiError> {
        Ok(Game { screen: Box::new(GlobalMapScreen::new(&config)?) })
    }

    pub fn run() -> Result<(), GameUiError> {
        let config = load_config()?;
        let context = Self::build_context(&config)?;

        main_loop(context, Game::new(config)?)?;
        Ok(())
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
            .build()?;

        Ok(context)
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.screen.tick(ctx);
    }
}
