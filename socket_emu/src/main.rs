use shtp::{DeviceType, SHTPServer};
mod socket;
use crate::socket::ElectricSocket;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 6411;

fn main() {
    match SHTPServer::new(
        HOST.to_string(),
        PORT,
        DeviceType::SmartSocket,
        ElectricSocket {},
    ) {
        Ok(server) => {
            println!("Listening on {}:{}", HOST, PORT);
            server.listen();
        }
        Err(error) => {
            println!("Server startup failed with error: {}", error)
        }
    }
}
