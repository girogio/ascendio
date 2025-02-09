use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AscendioError>;

#[derive(Error, Debug)]
pub enum AscendioError {
    #[error("Command deserialization error: {0}")]
    CommandDeserializationError(#[from] CommandError),
    #[error("Serial error: {0}")]
    SerialError(#[from] SerialError),
    #[error("Serialport error: {0}")]
    SerialportError(#[from] serialport::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("Device not found")]
    DeviceNotFound,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command type")]
    InvalidCommandType,
    #[error("Invalid length")]
    InvalidLength,
}

impl Serialize for AscendioError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AscendioError::SerialError(e) => e.to_string().serialize(serializer),
            AscendioError::CommandDeserializationError(e) => e.to_string().serialize(serializer),
            AscendioError::IoError(e) => e.to_string().serialize(serializer),
            AscendioError::SerialportError(e) => e.to_string().serialize(serializer),
            AscendioError::Unknown => "Unknown error".serialize(serializer),
        }
    }
}
