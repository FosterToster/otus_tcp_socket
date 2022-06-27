use std::io::{Write, Read};
use crate::abc::DeviceType;
use crate::error::SHTPError;
use crate::{send_shtp_request, receive_shtp_request, send_shtp_response, receive_shtp_response};

pub struct SHTPRequest {
    pub device_type: DeviceType,
    pub command: String,
    pub args: Vec<String>
}

impl SHTPRequest {
    pub fn send<T: Write>(&self, stream: &mut T) -> Result<(), SHTPError> {
        send_shtp_request(stream, self)
    }

    pub fn receive<T: Read>(stream: &mut T) -> Result<Self, SHTPError> {
        receive_shtp_request(stream)
    }
}

pub struct SHTPResponse {
    pub result: bool,
    pub data: String,
}

impl  SHTPResponse {
    pub fn done(data: &str) -> Self {
        Self { result: true, data: data.to_string() }
    }

    pub fn fail(data: &str) -> Self {
        Self { result: false, data: data.to_string() }
    }

    pub fn send<T: Write>(&self, stream: &mut T) -> Result<(), SHTPError> {
        send_shtp_response(stream, self)
    }

    pub fn receive<T: Read>(&self, stream: &mut T) -> Result<Self, SHTPError> {
        receive_shtp_response(stream)
    }
}

pub trait SHTPHandler {
    fn on_request(&self, request: &SHTPRequest) -> SHTPResponse;
}
