use std::io::{Write, Read};
use crate::abc::DeviceType;
use crate::error::SHTPError;
use crate::{send_shtp_request, receive_shtp_request};
use std::fmt::Display;

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
    pub data: String,
}

pub trait SHTPHandler {
    fn on_request(&self, request: &SHTPRequest) -> Result<SHTPResponse, Box<dyn Display>>;
}
