use crate::core::graphics::ContextWrapper;

pub struct DirectXContext {
    gl: glfw::Glfw
}

impl<'a> DirectXContext {
    pub fn new( gl: glfw::Glfw) -> *mut (dyn ContextWrapper + 'a) {
        &mut DirectXContext { gl: gl }
    }
}

impl ContextWrapper for DirectXContext {
    fn load_symbols(&mut self) {
        gl::load_with(|s| self.gl.get_proc_address_raw(s));
    }
}
