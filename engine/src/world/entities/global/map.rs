use std::collections::HashMap;

use crate::{
    errors::GameError,
    world::{entities::global::Tile, geometry::TilePos},
};

pub struct Map {
    tiles: HashMap<TilePos, Tile>,
    min_line: i32,
    max_line: i32,
    min_column: i32,
    max_column: i32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: HashMap::new(),
            min_line: 0,
            max_line: 0,
            min_column: 0,
            max_column: 0,
        }
    }

    pub fn width(&self) -> i32 {
        if self.tiles.is_empty() {
            return 0;
        }
        self.max_column - self.min_column + 1
    }

    pub fn height(&self) -> i32 {
        if self.tiles.is_empty() {
            return 0;
        }
        self.max_line - self.min_line + 1
    }

    pub fn add_tile(&mut self, tile: Tile) -> Result<(), GameError> {
        let tile_pos = tile.position().clone();
        if self.tiles.contains_key(&tile_pos) {
            return Err(GameError::TileAlreadyExists { position: tile_pos });
        } else {
            self.tiles.insert(tile_pos, tile);
            self.update_bounds(tile_pos);
        }
        Ok(())
    }

    fn update_bounds(&mut self, position: TilePos) {
        if self.tiles.len() == 1 {
            self.min_line = position.line();
            self.max_line = position.line();
            self.min_column = position.column();
            self.max_column = position.column();
        } else {
            if position.line() < self.min_line {
                self.min_line = position.line();
            } else if position.line() > self.max_line {
                self.max_line = position.line();
            }
            if position.column() < self.min_column {
                self.min_column = position.column();
            } else if position.column() > self.max_column {
                self.max_column = position.column();
            }
        }
    }
}
