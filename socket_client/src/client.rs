pub struct SHTPElectricSocketClient {
    client: shtp::SHTPClient,
}

impl SHTPElectricSocketClient {
    pub async fn new(host: String, port: u16) -> Self {
        Self {
            client: shtp::SHTPClient::new(host, port, shtp::DeviceType::SmartSocket).await,
        }
    }

    pub async fn send_any_command(
        &mut self,
        command: &str,
        args: Vec<String>,
    ) -> shtp::Result<shtp::SHTPResponse> {
        self.client.send_command(command.to_string(), args).await
    }
}
