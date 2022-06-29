use crate::device_type::DeviceType;
use crate::Result;
use crate::{receive_shtp_request, receive_shtp_response, send_shtp_request, send_shtp_response};
use std::io::{Read, Write};

pub struct SHTPRequest {
    pub device_type: DeviceType,
    pub command: String,
    pub args: Vec<String>,
}

impl SHTPRequest {
    pub fn send<T: Write>(&self, stream: &mut T) -> Result<()> {
        send_shtp_request(stream, self)
    }

    pub fn receive<T: Read>(stream: &mut T) -> Result<Self> {
        receive_shtp_request(stream)
    }
}

pub struct SHTPResponse {
    pub result: bool,
    pub data: String,
}

impl SHTPResponse {
    pub fn done(data: &str) -> Self {
        Self {
            result: true,
            data: data.to_string(),
        }
    }

    pub fn fail(data: &str) -> Self {
        Self {
            result: false,
            data: data.to_string(),
        }
    }

    pub fn send<T: Write>(&self, stream: &mut T) -> Result<()> {
        send_shtp_response(stream, self)
    }

    pub fn receive<T: Read>(&self, stream: &mut T) -> Result<Self> {
        receive_shtp_response(stream)
    }

    pub fn observe(&self) -> std::result::Result<&String, &String> {
        if self.result {
            Ok(&self.data)
        } else {
            Err(&self.data)
        }
    }
}

pub trait SHTPHandler {
    fn on_request(&self, request: &SHTPRequest) -> SHTPResponse;
}
