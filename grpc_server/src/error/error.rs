use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Failed to parse audio: {0}")]
    WavReaderError(
        #[source]
        #[from]
        hound::Error,
    ),
    #[error("Invalid audio: {0}")]
    InvalidAudio(String),
    #[error("VAD error: {0}")]
    VadError(
        #[source]
        #[from]
        silero_vad::error::error::ServiceError,
    ),
    #[error("Error: {0}")]
    Internal(
        #[source]
        #[from]
        Box<dyn std::error::Error + Send>,
    ),
}

pub type Result<T> = std::result::Result<T, ServiceError>;
