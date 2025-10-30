use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::errors::GameUiError;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub max_width: u32,
    pub zoom_step: u32,
}

#[derive(Debug, Deserialize)]
pub struct MapConfig {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub noise_params: NoiseParams,
}

#[derive(Debug, Deserialize)]
pub struct FontConfig {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct MenuRenderedConfig {
    pub top_border: u32,
    pub bottom_border: u32,
    pub left_border: u32,
    pub right_border: u32,
}

impl Default for MenuRenderedConfig {
    fn default() -> Self {
        Self {
            top_border: 1,
            bottom_border: 1,
            left_border: 1,
            right_border: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub camera: CameraConfig,
    pub map: MapConfig,
    pub surface_font: FontConfig,
    pub menu_renderer: MenuRenderedConfig,
}

pub fn load_config() -> Result<GameConfig, GameUiError> {
    let config = Config::builder()
        .add_source(File::new("config/config.json", FileFormat::Json))
        .build()?;

    Ok(config.try_deserialize()?)
}
