use bracket_lib::{
    color::{BLACK, RGB, WHITE},
    prelude::{BTerm, BTermBuilder, GameState, main_loop},
};
use engine::{
    services::world_building::{MapBuilder, MapBuilderFromHeights},
    world::entities::global::{SurfaceType, map::Map},
};

use crate::{
    config::{GameConfig, load_config},
    drawing::{
        Camera,
        tile_mapping::{SurfaceTile, TileMapper, build_surface_tile_mapper},
    },
    errors::GameUiError,
    utils::map_gen::generate_heightmap_f64_2d,
};

pub struct Game {
    map: Map,
    camera: Camera,
    surface_mapper: TileMapper<SurfaceTile>,
}

impl Game {
    pub fn new(config: GameConfig) -> Result<Self, GameUiError> {
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
            surface_mapper: build_surface_tile_mapper(),
        })
    }

    pub fn run() -> Result<(), GameUiError> {
        let config = load_config()?;
        let context = BTermBuilder::new()
            .with_title("Guild of Heroes")
            .with_resource_path("resources")
            .with_font(
                config.surface_font.path.clone(),
                config.surface_font.width,
                config.surface_font.height,
            )
            .with_tile_dimensions(
                config.surface_font.width,
                config.surface_font.height,
            )
            .with_dimensions(config.camera.width, config.camera.height)
            .with_simple_console(
                config.camera.width,
                config.camera.height,
                config.surface_font.path.clone(),
            )
            .build()?;

        main_loop(context, Game::new(config)?)?;
        Ok(())
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        for column in -1..self.camera.width() as i32 - 1 {
            for line in -1..self.camera.height() as i32 - 1 {
                let tile_surface = self
                    .map
                    .get_tile((line, column))
                    .map(|tile| tile.surface_type());
                ctx.set(
                    column + 1,
                    line + 1,
                    RGB::named(WHITE),
                    RGB::named(BLACK),
                    self.surface_mapper
                        .index(&SurfaceTile::from(tile_surface))
                        .unwrap(),
                );
            }
        }
    }
}
