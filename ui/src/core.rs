use engine::{
    services::world_building::{MapBuilder, MapBuilderFromHeights},
    world::entities::global::map::Map,
};

use crate::{
    config::load_config, errors::GameUiError,
    utils::map_gen::generate_heightmap_f64_2d,
};

struct Game {
    map: Map,
}

impl Game {
    pub fn new() -> Result<Self, GameUiError> {
        let config = load_config()?;
        let map_builder = MapBuilderFromHeights::new(
            generate_heightmap_f64_2d(&config.map)
                .iter()
                .map(|vec| vec.as_slice())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Ok(Game { map: map_builder.build()? })
    }
}
