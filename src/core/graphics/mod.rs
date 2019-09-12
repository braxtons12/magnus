#![allow(unconditional_recursion)]

pub mod opengl;
pub mod directx;
pub mod vulkan;

use std::error::Error;
use std::fmt;
use std::any::Any;
use std::sync::Arc;

use vulkano::swapchain::Surface;

pub struct Context<'a> {
    context_wrapper: *mut (dyn ContextWrapper<'a> + 'a)
}

impl<'a> Context<'a> {
    pub fn new(wrapper: Box<dyn ContextWrapper<'a> + 'a>) -> Context<'a> {
        Context { context_wrapper: Box::into_raw(wrapper) }
    }

    pub fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("Context loading symbols");
        unsafe {
            self.context_wrapper.as_mut().unwrap().load_symbols()
        }
    }

    pub fn create_devices(&mut self) -> Result<(), DeviceCreationError> {
        unsafe {
            self.context_wrapper.as_mut().unwrap().create_devices()
        }
    }
}

pub trait ContextWrapper<'a> {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError>;
    fn as_any(&'a mut self) -> &'a mut (dyn Any + 'a);
    fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>>;
    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError>;
}

impl<'a> ContextWrapper<'a> for &mut dyn ContextWrapper<'a> {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        self.load_symbols()
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        self.as_any()
    }

    fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        self.create_window_surface(glfw_w)
    }

    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        self.create_devices()
    }
}

impl<'a> ContextWrapper<'a> for *mut (dyn ContextWrapper<'a> + 'a) {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        unsafe {
            self.as_mut().unwrap().load_symbols()
        }
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        unsafe {
            self.as_mut().unwrap().as_any()
        }
    }

    fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        unsafe {
            self.as_mut().unwrap().create_window_surface(glfw_w)
        }
    }

    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        unsafe {
            self.as_mut().unwrap().create_devices()
        }
    }
}

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
