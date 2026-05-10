use std::path::PathBuf;

use spreet::SpreetError;
use spreet::resvg::usvg::Error as ResvgError;

#[derive(thiserror::Error, Debug)]
pub enum SpriteError {
    // Sprite source ID not found
    #[error("Sprite {0} not found")]
    SpriteNotFound(String),

    // IO error with file path
    #[error("IO error {0}: {1}")]
    IoError(std::io::Error, PathBuf),

    // Error during sprite processing
    #[error("{0} in file {1}")]
    SpriteProcessingError(SpreetError, PathBuf),

    // Error during sprite parsing
    #[error("{0} in file {1}")]
    SpriteParsingError(ResvgError, PathBuf),

    // Error during sprite sheet generation
    #[error("Unable to generate spritesheet")]
    UnableToGenerateSpritesheet,

    // Error during sprite instance creation
    #[error("Unable to create a sprite from file {0}")]
    SpriteInstError(PathBuf),

    // Directory does not exist
    #[error("Directory does not exist: {0}")]
    DirectoryNotFound(PathBuf),

    // Path is not a Directory
    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),
}
