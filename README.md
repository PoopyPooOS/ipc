# IPC Library

A very simple to use IPC library that can optionally handle multiple clients at once.

## Examples:
### Server
```rust
use ipc::{IpcError, Server};
use std::{
    io,
    thread::{self, sleep},
    time::Duration,
};

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

            // A `Client` struct is just a high level abstraction for a `UnixStream`
            client.send("Hello, world!")?;

            Ok(())
        })?;

        Ok(())
    });

    loop {
        println!("Running other tasks here while handling IPC in the background");
        sleep(Duration::from_secs(2));
    }
}
```

### Client
```rust
use ipc::{Client, IpcError};

fn main() -> Result<(), IpcError> {
    let mut client = Client::connect("example.sock")?;

    let message = client.receive::<String>()?;
    assert!(message == "Hello, world!");

    println!("Received: {message}");
    Ok(())
}
```

## Client Handling
There are 3 types of client handlers which can be enabled/disabled with features:
1. `clone-handler` (default): This feature adds a `Clone` trait bound for the client handler and clones the client handler for each thread.
2. `rwlock-handler`: This feature uses a `RwLock` for using the client handler between threads which doesn't require the handler to be cloneable but it uses a `Fn` instead of `FnMut` for the handler which means you can't mutate variables outside of the handler closure.
3. `sync-handler`: This feature makes the client handler synchronous which means that if 1 client is connected others cant send/receive data, but it most trait bounds from the handler which makes it easier to work with.

## Contributing
If not using vscode please make sure to copy all the rust-analyzer options from `.vscode/settings.json` otherwise you might get errors when using rust-analyzer.