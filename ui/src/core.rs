pub mod components;
mod screens;

use bracket_lib::prelude::{BTerm, BTermBuilder, GameState, main_loop};

const GLOBAL_MAP_CONSOLE_INDEX: usize = 0;
const MAIN_MENU_CONSOLE_INDEX: usize = 1;

use crate::{
    config::{GameConfig, load_config},
    core::screens::{MainMenuScreen, Screen},
    errors::{GameUiError, processing::process_ui_error},
};
pub struct Game {
    screen: Option<Box<dyn Screen>>,
    width: u32,
    height: u32,
    config: GameConfig,
}

impl Game {
    pub fn new(config: GameConfig) -> Result<Self, GameUiError> {
        Ok(Game {
            screen: Some(Box::new(MainMenuScreen::new())),
            width: config.width,
            height: config.height,
            config,
        })
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn config(&self) -> &GameConfig {
        &self.config
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
            .with_dimensions(config.width, config.height)
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
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(mut screen) = self.screen.take() {
            if let Err(error) = screen.tick(ctx, self) {
                process_ui_error(error);
                ctx.quitting = true;
            }
            if self.screen.is_none() {
                self.screen = Some(screen);
            }
        }
    }
}
