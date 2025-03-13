use serde::{Deserialize, Serialize};
use std::{io, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    #[error("Error decoding data: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Error encoding data: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("Connection closed unexpectedly")]
    ConnectionClosed,
    /// This error occurs if the CPU isn't 64bit and the buffer length prefix is too long.
    #[error("Data buffer length was truncated")]
    BufferLengthTruncated,
    /// This error mostly occurs when deserializing an [`IpcError`]
    #[error("Unknown error")]
    Unknown,
}

impl PartialEq for IpcError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (IpcError::Io(_), IpcError::Io(_))
                | (IpcError::Decode(_), IpcError::Decode(_))
                | (IpcError::Encode(_), IpcError::Encode(_))
                | (IpcError::ConnectionClosed, IpcError::ConnectionClosed)
                | (
                    IpcError::BufferLengthTruncated,
                    IpcError::BufferLengthTruncated
                )
                | (IpcError::Unknown, IpcError::Unknown)
        )
    }
}

impl FromStr for IpcError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(IpcError::Unknown)
    }
}

impl Serialize for IpcError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for IpcError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.parse().unwrap())
    }
}
