use crate::core::graphics::ContextWrapper;

pub struct OpenGLContext {
    gl: glfw::Glfw
}

impl<'a> OpenGLContext {
    pub fn new(gl: glfw::Glfw) -> *mut (dyn ContextWrapper +'a) {
        &mut OpenGLContext { gl: gl }
    }
}

impl ContextWrapper for OpenGLContext {
    fn load_symbols(&mut self) {
            debug!("OpenGL context loading symbols via gl.get_proc_address_raw()");
            gl::load_with(|s| self.gl.get_proc_address_raw(s));
    }
}
