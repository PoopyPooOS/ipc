use ipc::{IpcError, Server};
use logger::{error, info, trace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

fn main() -> Result<(), IpcError> {
    trace!("Starting server");
    let server = Server::new("example.sock")?;
    trace!("Starting receive loop");

    server.on_client(|mut client| {
        info!("Client connected");
        let received = client.read::<User>();

        match received {
            Ok(user) => {
                info!(format!("Got user: {user:#?}"));
                client.send(true).expect("Failed to send confirmation message to client");
            }
            Err(err) => {
                error!(format!("{err:#?}"));
                client.send(false).expect("Failed to send confirmation message to client");
            }
        }
    })?;

    info!("Server exiting");
    Ok(())
}
