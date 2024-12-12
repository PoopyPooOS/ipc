#[allow(
    unused_imports,
    reason = "This warning only appears when feature gates arent properly applied in an IDE"
)]
use crate::{client::Client, IpcError};
use logger::warn;
#[allow(
    unused_imports,
    reason = "This warning only appears when feature gates arent properly applied in an IDE"
)]
use std::{fs, os::unix::net::UnixListener, path::PathBuf, thread};

pub struct Server {
    #[allow(
        dead_code,
        reason = "This warning only appears when feature gates arent properly applied in an IDE"
    )]
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
    #[cfg(all(feature = "clone-handler", not(any(feature = "rwlock-handler", feature = "sync-handler"))))]
    pub fn on_client<F>(&self, handler: F) -> Result<(), IpcError>
    where
        F: FnMut(Client) -> Result<(), IpcError> + Clone + Send + 'static,
    {
        for stream in self.listener.incoming() {
            let client = Client::from_stream(stream?);
            let mut handler = handler.clone();

            thread::spawn(move || {
                let _ = handler(client);
            });
        }

        Ok(())
    }

    /// Add a handler for incoming connections.
    /// # Errors
    /// This function will return an error if the stream cannot be used.
    #[cfg(all(feature = "rwlock-handler", not(any(feature = "clone-handler", feature = "sync-handler"))))]
    pub fn on_client<F>(&self, handler: F) -> Result<(), IpcError>
    where
        F: Fn(Client) -> Result<(), IpcError> + Send + Sync + 'static,
    {
        use std::sync::{Arc, RwLock};

        let handler = Arc::new(RwLock::new(handler));

        for stream in self.listener.incoming() {
            let client = Client::from_stream(stream?);
            let handler = Arc::clone(&handler);

            thread::spawn(move || {
                let handler = handler.read().unwrap();
                let _ = handler(client);
            });
        }

        Ok(())
    }

    /// Add a handler for incoming connections.
    /// # Errors
    /// This function will return an error if the stream cannot be used.
    #[cfg(all(feature = "sync-handler", not(any(feature = "clone-handler", feature = "rwlock-handler"))))]
    pub fn on_client<F>(&self, handler: F) -> Result<(), IpcError>
    where
        F: Fn(Client) -> Result<(), IpcError> + Send + Sync + 'static,
    {
        for stream in self.listener.incoming() {
            let client = Client::from_stream(stream?);

            let _ = handler(client);
        }

        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        fs::remove_file(&self.socket_path).unwrap_or_else(|err| warn!(format!("Failed to clean up socket file: {err}")));
    }
}
