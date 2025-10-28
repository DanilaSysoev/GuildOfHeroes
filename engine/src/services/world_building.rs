use std::collections::HashMap;

use crate::{
    errors::GameError,
    world::{
        entities::global::{SurfaceType, Tile, map::Map},
        geometry::TilePos,
    },
};

pub trait MapBuilder {
    fn build(&self) -> Result<Map, GameError>;
}

pub struct SurfaceTypeHeightsInteval {
    min: f64,
    max: f64,
}

impl SurfaceTypeHeightsInteval {
    pub fn new(min: f64, max: f64) -> Self {
        SurfaceTypeHeightsInteval { min, max }
    }
    pub fn contains(&self, height: f64) -> bool {
        height >= self.min && height <= self.max
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
}

pub struct MapBuilderFromHeights {
    data: Vec<Vec<f64>>,
    surface_types_intervals: HashMap<SurfaceType, SurfaceTypeHeightsInteval>,
    default_surface_type: SurfaceType,
}

impl MapBuilderFromHeights {
    pub fn new(data: &[&[f64]]) -> Self {
        let mut map = MapBuilderFromHeights {
            data: data.iter().map(|slice| slice.to_vec()).collect(),
            surface_types_intervals: HashMap::new(),
            default_surface_type: Self::DEFAULT_SURFACE_TYPE,
        };
        map.build_default_intervals();
        map
    }

    pub fn with_default_surface_type(
        mut self,
        default_surface_type: SurfaceType,
    ) -> Self {
        self.default_surface_type = default_surface_type;
        self
    }

    pub fn with_surface_type_interval(
        mut self,
        surface_type: SurfaceType,
        min_height: f64,
        max_height: f64,
    ) -> Self {
        self.add_surface_tupe_interval(surface_type, min_height, max_height);
        self
    }

    const DEFAULT_SURFACE_TYPE: SurfaceType = SurfaceType::Water;

    fn build_default_intervals(&mut self) {
        self.add_surface_tupe_interval(SurfaceType::Water, 0.0, 0.2);
        self.add_surface_tupe_interval(SurfaceType::Swamp, 0.2, 0.3);
        self.add_surface_tupe_interval(SurfaceType::Ground, 0.3, 0.6);
        self.add_surface_tupe_interval(SurfaceType::Hill, 0.6, 0.7);
        self.add_surface_tupe_interval(SurfaceType::Forest, 0.7, 0.9);
        self.add_surface_tupe_interval(SurfaceType::Mountain, 0.9, 1.0);
    }

    fn add_surface_tupe_interval(
        &mut self,
        surface_type: SurfaceType,
        min_height: f64,
        max_height: f64,
    ) {
        self.surface_types_intervals.insert(
            surface_type,
            SurfaceTypeHeightsInteval { min: min_height, max: max_height },
        );
    }

    fn extract_surface_type(&self, height: f64) -> SurfaceType {
        *self
            .surface_types_intervals
            .iter()
            .find(|(_, interval)| interval.contains(height))
            .map(|(surface_type, _)| surface_type)
            .unwrap_or(&self.default_surface_type)
    }
}

impl MapBuilder for MapBuilderFromHeights {
    fn build(&self) -> Result<Map, GameError> {
        let mut map = Map::new();
        for (line_idx, line) in self.data.iter().enumerate() {
            for (column_idx, height) in line.iter().enumerate() {
                map.add_tile(Tile::new(
                    TilePos::new(line_idx as i32, column_idx as i32),
                    self.extract_surface_type(*height),
                ))?;
            }
        }
        Ok(map)
    }
}
