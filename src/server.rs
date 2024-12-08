use crate::{client::Client, IpcError};
use logger::warn;
use std::{fs, os::unix::net::UnixListener, path::PathBuf, thread};

pub struct Server {
    listener: UnixListener,
    socket_path: PathBuf,
}

impl Server {
    /// Bind a socket to a path.
    /// # Errors
    /// This function will return an error if binding the socket fails.
    pub fn new(socket_path: impl Into<PathBuf>) -> Result<Self, IpcError> {
        let socket_path = socket_path.into();
        let listener = UnixListener::bind(&socket_path)?;
        Ok(Self { listener, socket_path })
    }

    /// Add a handler for incoming connections.
    /// # Errors
    /// This function will return an error if the stream cannot be used.
    pub fn on_client<F>(&self, mut handler: F) -> Result<(), IpcError>
    where
        F: FnMut(Client) + Send + Copy + 'static,
    {
        for stream in self.listener.incoming() {
            let client = Client::from_stream(stream?);
            thread::spawn(move || {
                handler(client);
            });
        }

        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        fs::remove_file(&self.socket_path).unwrap_or_else(|err| warn!(format!("Failed to clean up socket file: {err}")));
    }
}
