use crate::device_type::DeviceType;
use crate::Result;
use crate::{receive_shtp_request, receive_shtp_response, send_shtp_request, send_shtp_response};
// use std::io::{Read, Write};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct SHTPRequest {
    pub device_type: DeviceType,
    pub command: String,
    pub args: Vec<String>,
}

impl SHTPRequest {
    pub async fn send<T: AsyncWrite + Unpin>(&self, stream: &mut T) -> Result<()> {
        send_shtp_request(stream, self).await
    }

    pub async fn receive<T: AsyncRead + Unpin>(stream: &mut T) -> Result<Self> {
        receive_shtp_request(stream).await
    }
}

pub struct SHTPResponse {
    pub result: bool,
    pub data: String,
}

impl SHTPResponse {
    pub async fn done(data: &str) -> Self {
        Self {
            result: true,
            data: data.to_string(),
        }
    }

    pub async fn fail(data: &str) -> Self {
        Self {
            result: false,
            data: data.to_string(),
        }
    }

    pub async fn send<T: AsyncWrite + Unpin>(&self, stream: &mut T) -> Result<()> {
        send_shtp_response(stream, self).await
    }

    pub async fn receive<T: AsyncRead + Unpin>(&self, stream: &mut T) -> Result<Self> {
        receive_shtp_response(stream).await
    }

    pub fn observe(&self) -> std::result::Result<&String, &String> {
        if self.result {
            Ok(&self.data)
        } else {
            Err(&self.data)
        }
    }
}

#[async_trait::async_trait]
pub trait SHTPHandler {
    async fn on_request(&self, request: &SHTPRequest) -> SHTPResponse;
}
