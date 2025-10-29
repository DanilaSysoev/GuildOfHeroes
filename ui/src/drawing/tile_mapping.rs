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

#[derive(Eq, PartialEq, Hash)]
pub enum SurfaceTile {
    Surface { surface: SurfaceType },
    Unknown,
}

impl From<Option<&SurfaceType>> for SurfaceTile {
    fn from(surface: Option<&SurfaceType>) -> Self {
        match surface {
            Some(s) => SurfaceTile::Surface { surface: *s },
            None => SurfaceTile::Unknown,
        }
    }
}

pub fn build_surface_tile_mapper() -> TileMapper<SurfaceTile> {
    TileMapper::new()
        .add(SurfaceTile::from(Some(&SurfaceType::Water)), 0)
        .add(SurfaceTile::from(Some(&SurfaceType::Swamp)), 1)
        .add(SurfaceTile::from(Some(&SurfaceType::Ground)), 2)
        .add(SurfaceTile::from(Some(&SurfaceType::Hill)), 3)
        .add(SurfaceTile::from(Some(&SurfaceType::Forest)), 4)
        .add(SurfaceTile::from(Some(&SurfaceType::Mountain)), 5)
        .add(SurfaceTile::Unknown, 19)
}
