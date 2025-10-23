use crate::world::geometry::TilePos;

pub enum GameError {
    TileAlreadyExists { position: TilePos },
}
