use ipc::{Client, IpcError};
use logger::{info, trace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

fn main() -> Result<(), IpcError> {
    trace!("Connecting to server");
    let mut client = Client::connect("example.sock")?;
    trace!("Connected to server");

    info!("Sending message");
    client.send(User {
        username: "John Doe".to_string(),
        password: "password123".to_string(),
    })?;
    info!("Sent message, waiting for response");

    let ok = client.read::<bool>()?;
    info!(format!("Received confirmation: {ok}"));

    if ok {
        info!("Server received message");
    } else {
        info!("Server did not receive message");
    }

    Ok(())
}
