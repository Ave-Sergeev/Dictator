use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscribeServiceError {
    #[error("Failed to parse audio: {0}")]
    WavReaderError(
        #[source]
        #[from]
        hound::Error,
    ),
    #[error("Invalid audio: {0}")]
    InvalidAudio(String),
}

pub type Result<T> = std::result::Result<T, TranscribeServiceError>;
