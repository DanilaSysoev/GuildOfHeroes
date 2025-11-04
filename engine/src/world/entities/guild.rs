pub mod builders;
mod hero;
mod item;
mod mission;

pub use hero::Hero;
pub use item::Item;
pub use mission::{Mission, MissionContext};

struct Guild {
    heroes: Vec<Hero>,
    items: Vec<Item>,
    money: i32,
}

impl Guild {
    pub fn new(start_money: i32) -> Self {
        Guild { heroes: Vec::new(), items: Vec::new(), money: start_money }
    }
}
