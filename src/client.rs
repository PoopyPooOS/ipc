use crate::IpcError;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    path::Path,
};

#[derive(Debug)]
pub struct Client {
    stream: UnixStream,
}

impl Client {
    /// Connect to a server.
    /// # Errors
    /// This function will return an error if the connection fails.
    pub fn connect(path: impl AsRef<Path>) -> Result<Self, IpcError> {
        let stream = UnixStream::connect(path)?;
        Ok(Self { stream })
    }

    #[must_use]
    pub fn from_stream(stream: UnixStream) -> Self {
        Client { stream }
    }

    /// Write data to the stream.
    /// # Errors
    /// This function will return an error if serialization fails.
    pub fn send<T: Serialize>(&mut self, data: T) -> Result<(), IpcError> {
        let serialized_data = bincode::serialize(&data)?;

        self.stream.write_all(&serialized_data)?;
        self.stream.flush()?;
        self.stream.shutdown(Shutdown::Write)?;

        Ok(())
    }

    /// Read data from the stream.
    /// # Errors
    /// This function will return an error if deserialization fails.
    pub fn read<T: for<'de> Deserialize<'de>>(&mut self) -> Result<T, IpcError> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer)?;
        bincode::deserialize(&buffer).map_err(Into::into)
    }

    /// Returns whether or not the client is connected.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.stream.peer_addr().is_ok()
    }
}
