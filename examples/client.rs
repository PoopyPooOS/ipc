use ipc::{Client, IpcError};

fn main() -> Result<(), IpcError> {
    let mut client = Client::connect("example.sock")?;

    loop {
        let message = client.receive::<String>()?;
        assert!(message == "Hello, world!");

        println!("Received: {message}");
    }
}
