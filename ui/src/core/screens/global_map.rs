use bracket_lib::{
    color::{BLACK, RGB, RGBA, WHITE},
    prelude::{BTerm, VirtualKeyCode},
};
use engine::{
    services::world_building::{MapBuilder, MapBuilderFromHeights},
    world::entities::global::map::Map,
};

use crate::{
    config::GameConfig,
    core::{
        GLOBAL_MAP_CONSOLE_INDEX, Game, MAIN_MENU_CONSOLE_INDEX,
        screens::Screen,
    },
    drawing::{
        Camera, Direction,
        tile_mapping::{SurfaceTile, TileMapper, build_surface_tile_mapper},
    },
    errors::GameUiError,
    utils::map_gen::generate_heightmap_f64_2d,
};

pub struct GlobalMapScreen {
    map: Map,
    camera: Camera,
    surface_mapper: TileMapper<SurfaceTile>,
}

impl GlobalMapScreen {
    pub fn new(config: &GameConfig) -> Result<Self, GameUiError> {
        let map_builder = MapBuilderFromHeights::new(
            generate_heightmap_f64_2d(&config.map)
                .iter()
                .map(|vec| vec.as_slice())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Ok(GlobalMapScreen {
            map: map_builder.build()?,
            camera: Camera::new(
                config.camera.width,
                config.camera.min_width,
                config.camera.max_width,
                config.camera.height,
                config.camera.zoom_step,
            )?,
            surface_mapper: build_surface_tile_mapper(),
        })
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => ctx.quitting = true,
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    self.camera.shift(Direction::Up)
                },
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    self.camera.shift(Direction::Down)
                },
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    self.camera.shift(Direction::Left)
                },
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.camera.shift(Direction::Right)
                },
                VirtualKeyCode::Space => {
                    self.camera.zoom_reset();
                },
                VirtualKeyCode::NumpadAdd => {
                    self.camera.zoom_in();
                },
                VirtualKeyCode::NumpadSubtract => {
                    self.camera.zoom_out();
                },
                _ => {},
            }
        }
    }

    fn draw(&mut self, ctx: &mut BTerm) {
        self.draw_map(ctx);
    }

    fn draw_map(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(GLOBAL_MAP_CONSOLE_INDEX);
        ctx.cls();
        ctx.set_char_size(self.camera.width(), self.camera.height());
        for column in 0..self.camera.width() as i32 {
            for line in 0..self.camera.height() as i32 {
                let tile_surface = self
                    .map
                    .get_tile((
                        self.camera.line_to_world(line),
                        self.camera.column_to_world(column),
                    ))
                    .map(|tile| tile.surface_type());
                ctx.set(
                    column,
                    line,
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

impl Screen for GlobalMapScreen {
    fn tick(&mut self, ctx: &mut BTerm, _: &mut Game) {
        self.handle_input(ctx);
        self.draw(ctx);
    }
}
