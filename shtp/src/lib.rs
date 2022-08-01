// use std::io::{Read, Write};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

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

pub async fn send_shtp_request<T: AsyncWrite + Unpin>(
    stream: &mut T,
    request: &handler::SHTPRequest,
) -> Result<()> {
    // handshake
    send_sized(stream, "shtp".as_bytes()).await?;
    send_sized(stream, &[1u8]).await?;
    // body
    send_device_type(stream, &request.device_type).await?;
    send_message(
        stream,
        serialize_message(&request.command, &request.args).await,
    )
    .await
}

pub async fn receive_shtp_request<T: AsyncRead + Unpin>(
    stream: &mut T,
) -> Result<handler::SHTPRequest> {
    try_handshake(stream).await?;
    let device_type = receive_device_type(stream).await?;
    let (command, args) = parse_message(read_message(stream).await?).await;
    Ok(handler::SHTPRequest {
        device_type,
        command,
        args,
    })
}

pub async fn send_shtp_response<T: AsyncWrite + Unpin>(
    stream: &mut T,
    response: &handler::SHTPResponse,
) -> Result<()> {
    // handshake
    send_sized(stream, "shtp".as_bytes()).await?;
    send_sized(stream, &[1u8]).await?;

    if response.result {
        send_sized(stream, &[1u8]).await?;
    } else {
        send_sized(stream, &[0u8]).await?;
    }

    send_message(stream, response.data.clone()).await
}

pub async fn receive_shtp_response<T: AsyncRead + Unpin>(
    stream: &mut T,
) -> Result<handler::SHTPResponse> {
    try_handshake(stream).await?;

    Ok(handler::SHTPResponse {
        result: read_result(stream).await?,
        data: read_message(stream).await?,
    })
}
