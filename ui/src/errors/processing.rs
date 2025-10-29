use std::error::Error;

use crate::errors::GameUiError;

pub fn process_ui_error(error: GameUiError) {
    match error {
        GameUiError::LoadConfig { error } => process_load_config_error(error),
        GameUiError::Engine { error } => process_game_engine_error(error),
        GameUiError::System { error } => process_system_error(error),
        GameUiError::Io { error } => process_io_error(error),
        GameUiError::Image { error } => process_image_error(error),
        GameUiError::BracketLib { error } => process_bracket_lib_error(error),
    }
}

fn process_load_config_error(error: config::ConfigError) {
    println!("Error during loading config: {:?}", error);
}

fn process_game_engine_error(error: engine::errors::GameError) {
    println!("Game error: {:?}", error)
}

fn process_system_error(error: Box<dyn Error>) {
    println!("Unexpected error: {:?}", error)
}

fn process_io_error(error: std::io::Error) {
    println!("IO error: {:?}", error)
}

fn process_image_error(error: image::ImageError) {
    println!("Image error: {:?}", error)
}

fn process_bracket_lib_error(
    error: Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
) {
    println!("Bracket lib error: {:?}", error);
}
