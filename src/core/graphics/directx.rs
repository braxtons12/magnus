use std::any::Any;
use std::sync::Arc;

use vulkano::swapchain::Surface;

use crate::core::graphics::{ContextWrapper, SymbolLoadError, DeviceCreationError};

pub struct DirectXContext {
    gl: glfw::Window
}

impl<'a> DirectXContext {
    pub fn new( gl: glfw::Window) -> Box<DirectXContext> {
        Box::from(DirectXContext { gl: gl })
    }
}

impl<'a> ContextWrapper<'a> for DirectXContext {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("DirectX context loading symbols with __");
        gl::load_with(|s| self.gl.glfw.get_proc_address_raw(s));
        if !gl::ClearColor::is_loaded() {
            return Err(SymbolLoadError::new("Failed to load OpenGL symbols"));
        } else {
            return Ok(());
        }
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        self
    }

    fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        None
    }

    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        Err(DeviceCreationError::NotVulkanContext)
    }
}
