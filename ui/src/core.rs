use engine::{
    services::world_building::{MapBuilder, MapBuilderFromHeights},
    world::entities::global::map::Map,
};

use crate::{
    config::load_config, drawing::Camera, errors::GameUiError,
    utils::map_gen::generate_heightmap_f64_2d,
};

struct Game {
    map: Map,
    camera: Camera,
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
        Ok(Game {
            map: map_builder.build()?,
            camera: Camera::new(config.camera.width, config.camera.height),
        })
    }

    pub fn run(&mut self) -> Result<(), GameUiError> {
        Ok(())
    }
}
