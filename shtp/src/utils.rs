use crate::device_type;
use crate::error;
use crate::Result;
// use std::io::{Read, Write};
use std::fmt::Write as _;
use std::marker::Unpin;
use tokio::io::AsyncReadExt;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

const DELIMITER: char = '|';

pub async fn try_handshake<T: AsyncRead + Unpin>(stream: &mut T) -> Result<()> {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf).await?;

    match &buf {
        b"shtp" => {
            let mut buf = [0];
            stream.read_exact(&mut buf).await?;
            match &buf {
                [1u8] => Ok(()),
                _ => Err(error::SHTPError::BadHandshake),
            }
        }
        _ => Err(error::SHTPError::BadHandshake),
    }
}

pub async fn receive_device_type<T: AsyncRead + Unpin>(
    stream: &mut T,
) -> Result<device_type::DeviceType> {
    let mut buf = [0; std::mem::size_of::<usize>()];
    stream.read_exact(&mut buf).await?;
    let device_type_len = usize::from_le_bytes(buf);

    let mut buf = vec![0; device_type_len];
    stream.read_exact(&mut buf).await?;

    device_type::DeviceType::try_from(
        String::from_utf8(buf)
            .map_err(|_| error::SHTPError::BadEncoding)?
            .as_ref(),
    )
}

pub async fn send_sized<T: AsyncWrite + Unpin>(stream: &mut T, buf: &[u8]) -> Result<()> {
    Ok(stream.write_all(buf).await?)
}

pub async fn send_device_type<T: AsyncWrite + Unpin>(
    stream: &mut T,
    device_type: &device_type::DeviceType,
) -> Result<()> {
    let str_device_type = String::from(device_type);
    let device_type_bytes = str_device_type.as_bytes();

    send_sized(stream, &device_type_bytes.len().to_le_bytes()).await?;
    send_sized(stream, device_type_bytes).await?;

    Ok(())
}

pub async fn read_message<T: AsyncRead + Unpin>(stream: &mut T) -> Result<String> {
    let mut buf = [0; std::mem::size_of::<usize>()];
    stream.read_exact(&mut buf).await?;
    let message_len = usize::from_le_bytes(buf);
    let mut buf = vec![0; message_len];
    stream.read_exact(&mut buf).await?;

    String::from_utf8(buf).map_err(|_| error::SHTPError::BadEncoding)
}

pub async fn read_result<T: AsyncRead + Unpin>(stream: &mut T) -> Result<bool> {
    let mut buf = [0; 1];
    stream.read_exact(&mut buf).await?;

    match buf[0] {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub async fn send_message<T: AsyncWrite + Unpin>(stream: &mut T, message: String) -> Result<()> {
    let message_bytes = message.as_bytes();

    send_sized(stream, &message_bytes.len().to_le_bytes()).await?;
    send_sized(stream, message_bytes).await?;

    Ok(())
}

pub async fn parse_message(message: String) -> (String, Vec<String>) {
    let mut command = String::new();
    let mut args = Vec::new();

    message
        .split(DELIMITER)
        .enumerate()
        .for_each(|(i, member)| {
            if i == 0 {
                command = member.to_string()
            } else if !member.is_empty() {
                args.push(member.to_string())
            }
        });

    (command, args)
}

pub async fn serialize_message(command: &String, args: &Vec<String>) -> String {
    let mut result = String::new();
    std::iter::once(command)
        .chain(args)
        .for_each(|member| write!(result, "{}{}", member, DELIMITER).unwrap());

    result
}
