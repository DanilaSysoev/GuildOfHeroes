mod global_map;
mod main_menu;

use bracket_lib::prelude::BTerm;
pub use global_map::GlobalMapScreen;

use crate::core::Game;

pub trait Screen {
    fn tick(&mut self, ctx: &mut BTerm, game: &mut Game);
}
