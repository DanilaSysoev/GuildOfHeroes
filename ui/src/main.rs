mod config;
mod drawing;
mod utils;

#[cfg(test)]
mod tests;

use crate::utils::map_gen::generate_heightmap_f64_2d;
use clap::{Parser, ValueEnum};
use config::load_config;
use engine::services::world_building::MapBuilderFromHeights;
use utils::map_gen::{
    NoiseParams, generate_heightmap_f64, save_heightmap_csv,
    save_heightmap_png,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Mode {
    ReadConfig,
    BuildTestMap,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum, short, long, default_value_t = Mode::ReadConfig)]
    mode: Mode,
}

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.mode {
        Mode::ReadConfig => read_config_demo(),
        Mode::BuildTestMap => build_test_map(),
    }
}

fn read_config_demo() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    println!("Camera width: {}", config.camera.width);
    println!("Camera height: {}", config.camera.height);

    Ok(())
}

fn build_test_map() -> Result<(), Box<dyn std::error::Error>> {
    let width = 128;
    let height = 128;
    let seed = 42_u64;
    let params = NoiseParams::default();

    let map = generate_heightmap_f64(width, height, seed, params);

    save_heightmap_png("data/map.png", width, height, &map)?;
    save_heightmap_csv("data/map.csv", width, height, &map)?;

    println!("Saved: map.png, map.csv");

    let _ = MapBuilderFromHeights::new(
        generate_heightmap_f64_2d(
            width as usize,
            height as usize,
            seed,
            params,
        )
        .iter()
        .map(|vec| vec.as_slice())
        .collect::<Vec<_>>()
        .as_slice(),
    );
    Ok(())
}
