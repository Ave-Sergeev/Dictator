use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Configuration error: {0}")]
    InvalidConfiguration(String),
    #[error("Silero error: {0}")]
    SileroError(String),
    #[error("Vad error: {0}")]
    VadError(String),
    #[error("Onnx error: {0}")]
    OnnxError(
        #[source]
        #[from]
        ort::Error,
    ),
}
