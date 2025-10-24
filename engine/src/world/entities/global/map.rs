use std::collections::{BTreeMap, HashMap, hash_map::Entry as HMEntry};

use crate::{
    errors::GameError,
    world::{entities::global::Tile, geometry::TilePos},
};

#[derive(Default)]
pub struct Map {
    tiles: HashMap<TilePos, Tile>,
    lines: BTreeMap<i32, u32>,
    columns: BTreeMap<i32, u32>,
    min_line: i32,
    max_line: i32,
    min_column: i32,
    max_column: i32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: HashMap::new(),
            lines: BTreeMap::new(),
            columns: BTreeMap::new(),
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
        let tile_pos = *tile.position();
        if self.tiles.contains_key(&tile_pos) {
            return Err(GameError::TileAlreadyExists { position: tile_pos });
        }

        self.tiles.insert(tile_pos, tile);
        self.update_structures_when_inserting(tile_pos);
        Ok(())
    }

    #[rustfmt::skip]
    pub fn set_tile(&mut self, tile: Tile) {
        let tile_pos = *tile.position();
        if let HMEntry::Vacant(entry) =
            self.tiles.entry(tile_pos)
        {
            entry.insert(tile);
            self.update_structures_when_inserting(tile_pos);
        } else {
            *self.tiles.get_mut(&tile_pos).unwrap() = tile;
        }
    }

    pub fn get_tile(&self, position: impl Into<TilePos>) -> Option<&Tile> {
        let position = position.into();
        self.tiles.get(&position)
    }

    pub fn get_tile_mut(
        &mut self,
        position: impl Into<TilePos>,
    ) -> Option<&mut Tile> {
        let position = position.into();
        self.tiles.get_mut(&position)
    }

    pub fn is_tile_exists(&self, position: impl Into<TilePos>) -> bool {
        let position = position.into();
        self.tiles.contains_key(&position)
    }

    pub fn remove_tile(&mut self, position: impl Into<TilePos>) {
        let position = position.into();
        if self.tiles.remove(&position).is_some() {
            self.update_structures_when_removing(position);
        }
    }

    fn update_bounds(&mut self) {
        if self.tiles.is_empty() {
            return;
        }
        self.min_line = *self.lines.first_key_value().unwrap().0;
        self.max_line = *self.lines.last_key_value().unwrap().0;
        self.min_column = *self.columns.first_key_value().unwrap().0;
        self.max_column = *self.columns.last_key_value().unwrap().0;
    }

    fn update_structures_when_inserting(
        &mut self,
        position: impl Into<TilePos>,
    ) {
        let position = position.into();
        Self::update_btree_entry_when_inserting(
            &mut self.lines,
            position.line(),
        );
        Self::update_btree_entry_when_inserting(
            &mut self.columns,
            position.column(),
        );
        self.update_bounds();
    }

    fn update_btree_entry_when_inserting(
        btree: &mut BTreeMap<i32, u32>,
        key: i32,
    ) {
        btree.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }

    fn update_structures_when_removing(
        &mut self,
        position: impl Into<TilePos>,
    ) {
        let position = position.into();
        Self::update_btree_entry_when_remowing(
            &mut self.lines,
            position.line(),
        );
        Self::update_btree_entry_when_remowing(
            &mut self.columns,
            position.column(),
        );
        self.update_bounds();
    }

    fn update_btree_entry_when_remowing(
        btree: &mut BTreeMap<i32, u32>,
        key: i32,
    ) {
        btree.entry(key).and_modify(|e| *e -= 1);
        if let Some(value) = btree.get(&key)
            && *value == 0
        {
            btree.remove(&key);
        }
    }
}
