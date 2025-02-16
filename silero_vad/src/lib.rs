pub mod error;

pub mod service;
pub mod utils;

pub type OnnxSession = ort::session::Session;

pub type Result<T> = std::result::Result<T, error::error::ServiceError>;
