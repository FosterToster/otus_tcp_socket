use crate::error::SHTPError;
use std::convert::TryFrom;
use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum DeviceType {
    SmartSocket,
}

impl From<DeviceType> for String {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::SmartSocket => String::from("SmartSocket")            
        }
    }
}

impl From<&DeviceType> for String {
    fn from(value: &DeviceType) -> Self {
        match value {
            DeviceType::SmartSocket => String::from("SmartSocket")            
        }
    }
}

impl TryFrom<&str> for DeviceType {
    type Error = SHTPError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SmartSocket" => Ok(DeviceType::SmartSocket),
            _ => Err(SHTPError::BadDevice),
        }
    }
}
