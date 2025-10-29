use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use image::{GrayImage, Luma};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::config::MapConfig;
use crate::utils::ensure_parent_dirs;

pub fn generate_heightmap_f64(config: &MapConfig) -> Vec<f64> {
    let mut noise = FastNoise::seeded(config.seed);

    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_frequency(config.noise_params.frequency);
    noise.set_fractal_octaves(config.noise_params.octaves);
    noise.set_fractal_lacunarity(config.noise_params.lacunarity);
    noise.set_fractal_gain(config.noise_params.gain);

    let mut data = Vec::with_capacity((config.width * config.height) as usize);
    for y in 0..config.height {
        for x in 0..config.width {
            let v = noise.get_noise(x as f32, y as f32);
            data.push(v as f64);
        }
    }

    normalize_01_f64(&mut data);
    data
}

pub fn generate_heightmap_f64_2d(config: &MapConfig) -> Vec<Vec<f64>> {
    let generated = generate_heightmap_f64(config);
    let mut result = Vec::with_capacity(config.height as usize);

    for y in 0..config.height {
        result.push(
            generated
                .iter()
                .skip((y * config.width) as usize)
                .take(config.width as usize)
                .cloned()
                .collect(),
        );
    }

    result
}

pub fn save_heightmap_png<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
    data_01: &[f64],
) -> image::ImageResult<()> {
    ensure_parent_dirs(&path)?;

    assert_eq!(data_01.len(), (width * height) as usize);

    let mut img = GrayImage::new(width, height);
    for (i, pix) in img.pixels_mut().enumerate() {
        let v = (data_01[i] * 255.0).round().clamp(0.0, 255.0) as u8;
        *pix = Luma([v]);
    }
    img.save(path)
}

pub fn save_heightmap_csv<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
    data_01: &[f64],
) -> std::io::Result<()> {
    ensure_parent_dirs(&path)?;

    assert_eq!(data_01.len(), (width * height) as usize);

    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    for y in 0..height {
        let start = (y * width) as usize;
        let row = &data_01[start..start + width as usize];
        for (i, v) in row.iter().enumerate() {
            if i > 0 {
                w.write_all(b",")?;
            }
            write!(w, "{:.6}", v)?;
        }
        w.write_all(b"\n")?;
    }
    w.flush()
}

fn normalize_01_f64(data: &mut [f64]) {
    if data.is_empty() {
        return;
    }
    let mut min_v = f64::MAX;
    let mut max_v = f64::MIN;
    for &v in data.iter() {
        if v < min_v {
            min_v = v;
        }
        if v > max_v {
            max_v = v;
        }
    }
    let span = (max_v - min_v).max(1e-9);
    for v in data.iter_mut() {
        *v = (*v - min_v) / span;
    }
}
