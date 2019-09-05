use crate::core::graphics::{ContextWrapper, SymbolLoadError};

pub struct DirectXContext {
    gl: glfw::Glfw
}

impl<'a> DirectXContext {
    pub fn new( gl: glfw::Glfw) -> *mut (dyn ContextWrapper + 'a) {
        &mut DirectXContext { gl: gl }
    }
}

impl ContextWrapper for DirectXContext {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("DirectX context loading symbols with __");
        gl::load_with(|s| self.gl.get_proc_address_raw(s));
        if !gl::ClearColor::is_loaded() {
            return Err(SymbolLoadError::new("Failed to load OpenGL symbols"));
        } else {
            return Ok(());
        }
    }
}
