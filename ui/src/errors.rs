pub mod processing;

use std::{error::Error as SystemError, io::Error as IoError};

use config::ConfigError;
use engine::errors::GameError;
use image::ImageError;

#[derive(Debug)]
pub enum GameUiError {
    LoadConfig {
        error: ConfigError,
    },
    Engine {
        error: GameError,
    },
    System {
        error: Box<dyn SystemError>,
    },
    Io {
        error: IoError,
    },
    Image {
        error: ImageError,
    },
    BracketLib {
        error:
            Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
    },
}

impl From<GameError> for GameUiError {
    fn from(error: GameError) -> Self {
        GameUiError::Engine { error }
    }
}

impl From<ConfigError> for GameUiError {
    fn from(error: ConfigError) -> Self {
        GameUiError::LoadConfig { error }
    }
}

impl From<Box<dyn SystemError>> for GameUiError {
    fn from(error: Box<dyn SystemError>) -> Self {
        GameUiError::System { error }
    }
}

impl From<IoError> for GameUiError {
    fn from(error: IoError) -> Self {
        GameUiError::Io { error }
    }
}

impl From<ImageError> for GameUiError {
    fn from(error: ImageError) -> Self {
        GameUiError::Image { error }
    }
}

impl From<Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>>
    for GameUiError
{
    fn from(
        error: Box<
            dyn std::error::Error + std::marker::Send + std::marker::Sync,
        >,
    ) -> Self {
        GameUiError::BracketLib { error }
    }
}
