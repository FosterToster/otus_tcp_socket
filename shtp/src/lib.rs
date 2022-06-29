use std::io::{Read, Write};

mod client;
mod device_type;
mod error;
mod handler;
mod server;
mod utils;

pub use client::SHTPClient;
pub use device_type::DeviceType;
pub use error::Result;
pub use handler::{SHTPHandler, SHTPRequest, SHTPResponse};
pub use server::SHTPServer;

use utils::*;

pub fn send_shtp_request<T: Write>(stream: &mut T, request: &handler::SHTPRequest) -> Result<()> {
    // handshake
    send_sized(stream, "shtp".as_bytes())?;
    send_sized(stream, &[1u8])?;
    // body
    send_device_type(stream, &request.device_type)?;
    encode_message(stream, serialize_message(&request.command, &request.args))
}

pub fn receive_shtp_request<T: Read>(stream: &mut T) -> Result<handler::SHTPRequest> {
    try_handshake(stream)?;
    let device_type = receive_device_type(stream)?;
    let (command, args) = parse_message(read_message(stream)?);
    Ok(handler::SHTPRequest {
        device_type,
        command,
        args,
    })
}

pub fn send_shtp_response<T: Write>(
    stream: &mut T,
    response: &handler::SHTPResponse,
) -> Result<()> {
    // handshake
    send_sized(stream, "shtp".as_bytes())?;
    send_sized(stream, &[1u8])?;

    if response.result {
        send_sized(stream, &[1u8])?;
    } else {
        send_sized(stream, &[0u8])?;
    }

    send_sized(stream, response.data.as_bytes())
}

pub fn receive_shtp_response<T: Read>(stream: &mut T) -> Result<handler::SHTPResponse> {
    try_handshake(stream)?;

    Ok(handler::SHTPResponse {
        result: read_result(stream)?,
        data: read_message(stream)?,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
