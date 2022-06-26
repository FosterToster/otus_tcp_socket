use std::{io::{Read, Write}};

use abc::DeviceType;
use handler::SHTPRequest;
pub mod abc;
pub mod client;
pub mod error;
pub mod handler;
pub mod server;

const DELIMITER: char = '|';

pub fn try_handshake<T: Read>(stream: &mut T) -> Result<(), error::SHTPError> {
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    stream.read_exact(&mut buf)?;

    match &buf {
        b"shtp" => {
            let mut buf: [u8; 1] = [0];
            stream.read_exact(&mut buf)?;
            match &buf {
                [1u8] => Ok(()),
                _ => Err(error::SHTPError::BadHandshake),
            }
        }
        _ => Err(error::SHTPError::BadHandshake),
    }
}

fn decode_device_type<T: Read>(stream: &mut T) -> Result<abc::DeviceType, error::SHTPError> {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf)?;
    let device_type_len = u32::from_le_bytes(buf);
    let mut buf = vec![0; device_type_len as _];
    stream.read_exact(&mut buf)?;

    DeviceType::try_from(
        String::from_utf8(buf)
            .map_err(|_| error::SHTPError::BadEncoding)?
            .as_ref(),
    )
}

fn send_sized<T: Write>(stream: &mut T, buf: &[u8]) -> Result<(), error::SHTPError> {
    let size = stream.write(buf)?;

    if size != buf.len() {
        return Err(error::SHTPError::NotExhaused)
    }

    Ok(())
}

fn encode_device_type<T: Write>(stream: &mut T, device_type: &abc::DeviceType) -> Result<(), error::SHTPError> {
    let str_device_type: String = device_type.into();

    send_sized(stream, &str_device_type.len().to_le_bytes())?;
    send_sized(stream, str_device_type.as_bytes())?;

    Ok(())
}

fn decode_message<T: Read>(stream: &mut T) -> Result<String, error::SHTPError> {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf)?;
    let message_len = u32::from_le_bytes(buf);
    let mut buf = vec![0; message_len as _];
    stream.read_exact(&mut buf)?;

    String::from_utf8(buf)
        .map_err(|_| error::SHTPError::BadEncoding)
}

fn encode_message<T: Write>(stream: &mut T, message: String) -> Result<(), error::SHTPError> {
    send_sized(stream, &message.len().to_le_bytes())?;
    send_sized(stream, message.as_bytes())?;

    Ok(())
}

fn parse_message(message: String) -> (String, Vec<String>) {
    let mut command = String::new();
    let mut args = Vec::new();

    message.split(DELIMITER)
        .enumerate()
        .for_each(|(i, member)| {
            if i == 0 {
                command = member.to_string()
            } else {
                args.push(member.to_string())
            }
        });
    
    (command, args)

}

pub fn receive_shtp_request<T: Read>(stream: &mut T) -> Result<handler::SHTPRequest, error::SHTPError> {
    try_handshake(stream)?;
    let device_type = decode_device_type(stream)?;
    let (command, args) = parse_message( decode_message(stream)? );
    Ok(SHTPRequest {
        device_type,
        command,
        args
    })
}

fn serialize_message(command: &String, args: &Vec<String>) -> String {
    let mut result = String::new();
    std::iter::once(command)
        .chain(args)
        .for_each(|member| {
            result.push_str(&format!("{}{}", member, DELIMITER))
        });

    result

}

pub fn send_shtp_request<T: Write>(stream: &mut T, request: &handler::SHTPRequest) -> Result<(), error::SHTPError> {
    // handshake
    send_sized(stream, "shtp".as_bytes())?;
    send_sized(stream, &[1u8])?;
    // body
    encode_device_type(stream, &request.device_type)?;
    encode_message(stream, serialize_message(&request.command, &request.args))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
