use crate::core::window::*;
use crate::events::event::{EventCallbackFn, Event};

pub struct Win32Window {
    props: WindowProps,
    callback: EventCallbackFn,
    vsync: bool,
    window: glfw::Window
}

impl<'a> Win32Window {

    pub fn new(props: WindowProps, callback: EventCallbackFn, vsync: bool, window: glfw::Window) -> Box<dyn WindowBehavior + 'a> {
        Box::from(Win32Window { props: props, callback: callback, vsync: vsync, window: window })
    }

    pub fn on_update(e: &mut (dyn Event)) -> bool {
        let h = e.handled();
        *h = true;
        true
    }
}

impl WindowBehavior for Win32Window {

    fn get_width(&self) -> u32 {
        self.props.width
    }

    fn set_width(&mut self, w: u32) {
        self.props.width = w;
    }

    fn get_height(&self) -> u32 {
        self.props.height
    }

    fn set_height(&mut self, h: u32) {
        self.props.height = h;
    }

    fn set_event_callback(&mut self, func: EventCallbackFn) {
        self.callback = func;
    }

    fn is_vsync(&self) -> bool {
        self.vsync
    }

    fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }

    fn get_glfw_window(&self) -> & glfw::Window {
        &(self.window)
    }
}
