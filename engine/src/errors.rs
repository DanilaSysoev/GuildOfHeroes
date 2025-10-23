use crate::world::geometry::TilePos;

#[derive(Debug)]
pub enum GameError {
    TileAlreadyExists { position: TilePos },
}
