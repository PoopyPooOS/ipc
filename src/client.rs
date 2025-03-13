use crate::IpcError;
use bincode::config::standard;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};

#[derive(Debug)]
pub struct Client {
    pub stream: UnixStream,
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
        let serialized_data = bincode::serde::encode_to_vec(data, standard())?;
        let length = (serialized_data.len() as u64).to_be_bytes();

        let mut message = length.to_vec();
        message.extend_from_slice(&serialized_data);

        self.stream.write_all(&message)?;
        self.stream.flush()?;

        Ok(())
    }

    /// Read data from the stream.
    /// # Errors
    /// This function will return an error if deserialization fails.
    pub fn receive<T: for<'de> Deserialize<'de>>(&mut self) -> Result<T, IpcError> {
        let mut length_buffer = [0u8; 8];
        self.stream.read_exact(&mut length_buffer)?;
        let length = u64::from_be_bytes(length_buffer);
        let length = usize::try_from(length).map_err(|_| IpcError::BufferLengthTruncated)?;

        let mut buffer = vec![0; length];
        self.stream.read_exact(&mut buffer)?;

        bincode::serde::decode_from_slice(&buffer, standard())?.0
    }

    /// Returns whether or not the client is connected.
    /// Note: This usually doesnt work properly.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.stream.peer_addr().is_ok()
    }
}
