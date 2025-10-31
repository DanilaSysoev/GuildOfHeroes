mod global_map;
mod main_menu;

use bracket_lib::prelude::BTerm;
pub use global_map::GlobalMapScreen;
pub use main_menu::MainMenuScreen;

use crate::{core::Game, errors::GameUiError};

pub trait Screen {
    fn tick(
        &mut self,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError>;
}
