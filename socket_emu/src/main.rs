use shtp::{DeviceType, SHTPServer};
mod socket;
use crate::socket::ElectricSocket;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 6411;

#[tokio::main]
async fn main() {
    match SHTPServer::new(
        HOST.to_string(),
        PORT,
        DeviceType::SmartSocket,
        ElectricSocket::default(),
    )
    .await
    {
        Ok(server) => {
            println!("Listening on {}:{}", HOST, PORT);
            server.listen().await;
        }
        Err(error) => {
            println!("Server startup failed with error: {}", error)
        }
    }
}
