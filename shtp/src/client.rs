use crate::device_type::DeviceType;
use crate::handler::{SHTPRequest, SHTPResponse};
use crate::Result;
use crate::{receive_shtp_response, send_shtp_request};
// use std::net::TcpStream;
use tokio::net::TcpStream;

pub struct SHTPClient {
    host: String,
    port: u16,
    device_type: DeviceType,
}

impl SHTPClient {
    pub async fn new(host: String, port: u16, device_type: DeviceType) -> Self {
        Self {
            host,
            port,
            device_type,
        }
    }

    async fn connect(&self) -> Result<TcpStream> {
        Ok(TcpStream::connect(format!("{}:{}", self.host, self.port)).await?)
    }

    pub async fn send_command(&self, command: String, args: Vec<String>) -> Result<SHTPResponse> {
        let mut stream = self.connect().await?;

        send_shtp_request(
            &mut stream,
            &SHTPRequest {
                device_type: self.device_type,
                command,
                args,
            },
        )
        .await?;

        receive_shtp_response(&mut stream).await
    }
}
