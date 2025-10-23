use crate::world::geometry::TilePos;

pub enum SurfaceType {
    Water,
    Ground,
    Hill,
    Mountain,
    Forest,
    Swamp,
}

pub struct Tile {
    position: TilePos,
    surface_type: SurfaceType,
}

impl Tile {
    pub fn new(position: TilePos, surface_type: SurfaceType) -> Tile {
        Tile { position, surface_type }
    }

    pub fn position(&self) -> &TilePos {
        &self.position
    }

    pub fn line(&self) -> i32 {
        self.position().line()
    }

    pub fn column(&self) -> i32 {
        self.position().column()
    }

    pub fn surface_type(&self) -> &SurfaceType {
        &self.surface_type
    }
}
