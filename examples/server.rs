use ipc::{IpcError, Server};
use std::{io, thread, time::Duration};

fn main() -> Result<(), IpcError> {
    thread::spawn(move || {
        // Start server
        let server = match Server::new("example.sock") {
            Ok(server) => server,
            Err(IpcError::Io(ref io_err)) if io_err.kind() == io::ErrorKind::AddrInUse => {
                eprintln!("Error: server already running");
                return Ok(());
            }
            Err(err) => return Err(err),
        };

        println!("Waiting for clients...");

        // Set client handler
        server.on_client(move |mut client| {
            println!("Client connected");

            loop {
                // A `Client` struct is just a high level abstraction for a `UnixStream`
                client.send("Hello, world!")?;
            }
        })?;

        Ok(())
    });

    loop {
        println!("Running other tasks here while handling IPC in the background");
        thread::sleep(Duration::from_secs(2));
    }
}
