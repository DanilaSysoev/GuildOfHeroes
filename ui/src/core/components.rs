use bracket_lib::prelude::BTerm;

pub trait GameEntity {
    fn update(&mut self, ctx: &mut BTerm);
}
