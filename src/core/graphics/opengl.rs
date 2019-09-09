use std::any::Any;
use std::sync::Arc;

use vulkano::swapchain::Surface;

use crate::core::graphics::{ContextWrapper, SymbolLoadError, DeviceCreationError};

pub struct OpenGLContext {
    gl: glfw::Window
}

impl<'a> OpenGLContext {
    pub fn new(gl: glfw::Window) -> Box<OpenGLContext> {
        Box::from(OpenGLContext { gl })
    }
}

impl<'a> ContextWrapper<'a> for OpenGLContext {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("OpenGL context loading symbols via gl.get_proc_address_raw()");
        gl::load_with(|s| self.gl.glfw.get_proc_address_raw(s));
        if !gl::ClearColor::is_loaded() {
            Err(SymbolLoadError::new("Failed to load OpenGL symbols"))
        } else {
            Ok(())
        }
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        self
    }

    fn create_window_surface(&mut self, _glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        None
    }

    fn create_devices(&mut self) -> Result<(), DeviceCreationError> {
        Err(DeviceCreationError::NotVulkanContext)
    }
}
