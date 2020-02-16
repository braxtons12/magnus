pub mod opengl;
#[cfg(windows)]
pub mod directx;
pub mod vulkan;
pub mod context;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SymbolLoadError {
    details: String
}

impl SymbolLoadError {
    fn new(msg: &str) -> SymbolLoadError {
        SymbolLoadError { details: String::from(msg) }
    }
}

impl fmt::Display for SymbolLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SymbolLoadError {
    fn description(&self) -> & str {
        & self.details
    }
}

#[derive(Debug)]
pub enum DeviceCreationError {
    NotVulkanContext,
    FailedToCreateVulkanDevice
}

impl fmt::Display for DeviceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            DeviceCreationError::NotVulkanContext => "Not Vulkan Context",
            DeviceCreationError::FailedToCreateVulkanDevice => "Failed To Create Vulkan Device"
        })
    }
}

impl Error for DeviceCreationError {
    fn description(&self) -> & str {
        match self {
            DeviceCreationError::NotVulkanContext => "Not Vulkan Context",
            DeviceCreationError::FailedToCreateVulkanDevice => "Failed To Create Vulkan Device"
        }
    }
}
