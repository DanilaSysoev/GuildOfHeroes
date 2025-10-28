mod utils;

use engine::services::world_building::MapBuilderFromHeights;
use utils::map_gen::{
    NoiseParams, generate_heightmap_f64, save_heightmap_csv,
    save_heightmap_png,
};

use crate::utils::map_gen::generate_heightmap_f64_2d;

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 128;
    let height = 128;
    let seed = 42_u64;
    let params = NoiseParams::default();

    let map = generate_heightmap_f64(width, height, seed, params);

    save_heightmap_png("data/map.png", width, height, &map)?;
    save_heightmap_csv("data/map.csv", width, height, &map)?;

    println!("Saved: map.png, map.csv");

    let _ = 
        MapBuilderFromHeights::new(generate_heightmap_f64_2d(
            width as usize,
            height as usize,
            seed,
            params,
        ).iter()
         .map(|vec| vec.as_slice())
         .collect::<Vec<_>>()
         .as_slice());
    Ok(())
}
