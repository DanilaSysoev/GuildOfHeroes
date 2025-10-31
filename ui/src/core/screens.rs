mod global_map_screen;
mod main_menu_screen;

use bracket_lib::prelude::BTerm;
pub use global_map_screen::GlobalMapScreen;
pub use main_menu_screen::MainMenuScreen;

use crate::{core::Game, errors::GameUiError};

pub trait Screen {
    fn tick(
        &mut self,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError>;
}
