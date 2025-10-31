mod global_map;

use bracket_lib::prelude::BTerm;
pub use global_map::GlobalMapScreen;

pub trait Screen {
    fn tick(&mut self, ctx: &mut BTerm);
}
