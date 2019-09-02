use crate::core::graphics::{ContextWrapper, SymbolLoadError};

pub struct OpenGLContext {
    gl: glfw::Glfw
}

impl<'a> OpenGLContext {
    pub fn new(gl: glfw::Glfw) -> *mut (dyn ContextWrapper +'a) {
        &mut OpenGLContext { gl: gl }
    }
}

impl ContextWrapper for OpenGLContext {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("OpenGL context loading symbols via gl.get_proc_address_raw()");
        gl::load_with(|s| self.gl.get_proc_address_raw(s));
        if !gl::ClearColor::is_loaded() {
            return Err(SymbolLoadError::new("Failed to load OpenGL symbols"));
        } else {
            return Ok(());
        }
    }
}
