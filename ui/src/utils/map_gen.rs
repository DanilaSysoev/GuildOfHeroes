use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use image::{GrayImage, Luma};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct NoiseParams {
    pub frequency: f32,  // базовая частота, напр. 0.01..0.03
    pub octaves: i32,    // число октав, напр. 5..8
    pub lacunarity: f32, // во сколько раз растёт частота на октаву (обычно ~2.0)
    pub gain: f32, // во сколько уменьшается амплитуда на октаву (обычно ~0.4..0.6)
}

impl Default for NoiseParams {
    fn default() -> Self {
        Self { frequency: 0.02, octaves: 6, lacunarity: 2.0, gain: 0.45 }
    }
}

pub fn generate_heightmap_f64(
    width: u32,
    height: u32,
    seed: u64,
    params: NoiseParams,
) -> Vec<f64> {
    let mut noise = FastNoise::seeded(seed);

    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_frequency(params.frequency);
    noise.set_fractal_octaves(params.octaves);
    noise.set_fractal_lacunarity(params.lacunarity);
    noise.set_fractal_gain(params.gain);

    let mut data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let v = noise.get_noise(x as f32, y as f32);
            data.push(v as f64);
        }
    }

    normalize_01_f64(&mut data);
    data
}

pub fn save_heightmap_png<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
    data_01: &[f64],
) -> image::ImageResult<()> {
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
