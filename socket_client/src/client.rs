pub struct SHTPElectricSocketClient {
    client: shtp::SHTPClient,
}

impl SHTPElectricSocketClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            client: shtp::SHTPClient::new(host, port, shtp::DeviceType::SmartSocket),
        }
    }

    pub fn send_any_command(&mut self, command: &str) -> shtp::Result<shtp::SHTPResponse> {
        self.client.send_command(command.to_string(), vec![])
    }
}
