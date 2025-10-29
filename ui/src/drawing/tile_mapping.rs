use std::collections::HashMap;
use std::hash::Hash;

use engine::world::entities::global::SurfaceType;

#[derive(Default)]
pub struct TileMapper<T: Eq + Hash> {
    map: HashMap<T, u32>,
}

impl<T: Eq + Hash> TileMapper<T> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn add(mut self, tile: T, index: u32) -> Self {
        self.map.insert(tile, index);
        self
    }

    pub fn index(&self, tile: &T) -> Option<u32> {
        self.map.get(tile).copied()
    }
}

pub fn build_surface_tile_mapper() -> TileMapper<SurfaceType> {
    TileMapper::new()
        .add(SurfaceType::Water, 0)
        .add(SurfaceType::Swamp, 1)
        .add(SurfaceType::Ground, 2)
        .add(SurfaceType::Hill, 3)
        .add(SurfaceType::Forest, 4)
        .add(SurfaceType::Mountain, 5)
}
