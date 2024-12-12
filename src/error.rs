use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, io, str::FromStr};

#[derive(Debug)]
pub enum IpcError {
    Io(io::Error),
    Serialization(bincode::Error),
    ConnectionClosed,
    ConnectionError,
    /// This error occurs if the CPU isn't 64bit and the buffer length prefix is too long.
    BufferLengthTruncated,
    /// This error mostly occurs when deserializing an [`IpcError`]
    Unknown,
}

impl PartialEq for IpcError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (IpcError::Io(_), IpcError::Io(_))
                | (IpcError::Serialization(_), IpcError::Serialization(_))
                | (IpcError::ConnectionClosed, IpcError::ConnectionClosed)
                | (IpcError::ConnectionError, IpcError::ConnectionError)
                | (IpcError::BufferLengthTruncated, IpcError::BufferLengthTruncated)
                | (IpcError::Unknown, IpcError::Unknown)
        )
    }
}

impl Error for IpcError {
}

impl Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for IpcError {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(IpcError::Unknown)
    }
}

impl From<io::Error> for IpcError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<bincode::Error> for IpcError {
    fn from(value: bincode::Error) -> Self {
        Self::Serialization(value)
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
