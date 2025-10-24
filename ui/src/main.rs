mod utils;

use utils::map_gen::{
    NoiseParams, generate_heightmap_f64, save_heightmap_csv,
    save_heightmap_png,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 128;
    let height = 128;
    let seed = 42_u64;
    let params = NoiseParams::default();

    let map = generate_heightmap_f64(width, height, seed, params);

    save_heightmap_png("data/map.png", width, height, &map)?;
    save_heightmap_csv("data/map.csv", width, height, &map)?;

    println!("Saved: map.png, map.csv");
    Ok(())
}
