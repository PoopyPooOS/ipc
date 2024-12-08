use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum IpcError {
    Io(io::Error),
    Serialization(bincode::Error),
    ConnectionError,
}

impl Error for IpcError {
}

impl Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
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
