use crate::device_type::DeviceType;
use crate::handler::{SHTPRequest, SHTPResponse};
use crate::Result;
use crate::{receive_shtp_response, send_shtp_request};
use std::net::TcpStream;

pub struct SHTPClient {
    host: String,
    port: u16,
    device_type: DeviceType,
}

impl SHTPClient {
    pub fn new(host: String, port: u16, device_type: DeviceType) -> Result<Self> {
        Ok(Self {
            host,
            port,
            device_type,
        })
    }

    fn connect(&self) -> Result<TcpStream> {
        Ok(TcpStream::connect(format!("{}:{}", self.host, self.port))?)
    }

    pub fn send_command(&self, command: String, args: Vec<String>) -> Result<SHTPResponse> {
        let mut stream = self.connect()?;

        send_shtp_request(
            &mut stream,
            &SHTPRequest {
                device_type: self.device_type,
                command,
                args,
            },
        )?;

        receive_shtp_response(&mut stream)
    }
}
