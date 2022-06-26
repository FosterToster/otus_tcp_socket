use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SHTPError {
    #[error("Device type is not present")]
    UnknownDevice,
    #[error("Bad device type passed in request")]
    BadDevice,
    #[error("Client is not SHTP")]
    BadHandshake,
    #[error("Non UTF-8 data was passed")]
    BadEncoding,
    #[error("Stream read/Write error")]
    IoError(#[from] io::Error),
    #[error("Data was not fully exhausted")]
    NotExhaused
}
