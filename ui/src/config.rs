use anyhow::{Context, Ok, Result};
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub camera: CameraConfig,
}

pub fn load_config() -> Result<GameConfig> {
    let config = Config::builder()
        .add_source(File::new("config/config.json", FileFormat::Json))
        .build()
        .context(
            "Error during building configs from file \"config/config.json\"",
        )?;

    Ok(config
        .try_deserialize()
        .context("Error during deserialization of congig")?)
}
