use crate::world::geometry::TilePos;

#[derive(Debug)]
pub enum GameError {
    TileAlreadyExists { position: TilePos },
    HeroAttributeNotFound { name: String },
    HeroTraitNotFound { name: String },
    HeroAttributeFactoryAlreadyExists { name: String },
    HeroTraitFactroyAlreadyExists { name: String },
}
