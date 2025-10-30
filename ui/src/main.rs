mod config;
mod core;
mod drawing;
mod errors;
mod utils;
mod widgets;

#[cfg(test)]
mod tests;

use crate::{
    core::Game,
    errors::{GameUiError, processing::process_ui_error},
    utils::map_gen::generate_heightmap_f64_2d,
};
use clap::{Parser, ValueEnum};
use config::load_config;
use engine::services::world_building::MapBuilderFromHeights;
use utils::map_gen::{
    generate_heightmap_f64, save_heightmap_csv, save_heightmap_png,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Mode {
    ReadConfig,
    BuildTestMap,
    RunGame,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum, short, long, default_value_t = Mode::RunGame)]
    mode: Mode,
}

#[rustfmt::skip]
fn main() {
    let args = Args::parse();

    if let Err(error) = match args.mode {
        Mode::ReadConfig => read_config_demo(),
        Mode::BuildTestMap => build_test_map(),
        Mode::RunGame => run_game(),
    } {
        process_ui_error(error);
    }
}

fn read_config_demo() -> Result<(), GameUiError> {
    let config = load_config()?;

    println!("Camera width: {}", config.camera.width);
    println!("Camera height: {}", config.camera.height);

    Ok(())
}

fn build_test_map() -> Result<(), GameUiError> {
    let config = load_config()?;

    let map = generate_heightmap_f64(&config.map);

    save_heightmap_png(
        "data/map.png",
        config.map.width,
        config.map.height,
        &map,
    )?;
    save_heightmap_csv(
        "data/map.csv",
        config.map.width,
        config.map.height,
        &map,
    )?;

    println!("Saved: map.png, map.csv");

    let _ = MapBuilderFromHeights::new(
        generate_heightmap_f64_2d(&config.map)
            .iter()
            .map(|vec| vec.as_slice())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    Ok(())
}

fn run_game() -> Result<(), GameUiError> {
    Game::run()
}
