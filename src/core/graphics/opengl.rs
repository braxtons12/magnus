use std::fmt;

use crate::core::graphics::SymbolLoadError;

pub struct OpenGLContext {
    window: glfw::Window,
}

unsafe impl std::marker::Send for OpenGLContext {}
unsafe impl std::marker::Sync for OpenGLContext {}

impl fmt::Debug for OpenGLContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenGLContext {{window: width: {}, height: {} }}", self.window.get_size().0, self.window.get_size().1)
    }
}

impl OpenGLContext {
    pub fn new(window: glfw::Window) -> OpenGLContext {
        OpenGLContext { window }
    }

    pub fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("OpenGL context loading symbols via gl.get_proc_address_raw()");
        gl::load_with(|s| self.window.glfw.get_proc_address_raw(s));
        if !gl::ClearColor::is_loaded() {
            Err(SymbolLoadError::new("Failed to load OpenGL symbols"))
        } else {
            Ok(())
        }
    }

    pub fn get_window(&mut self) -> &mut glfw::Window {
        &mut self.window
    }
}

