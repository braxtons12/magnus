use crate::core::window::*;
use crate::core::graphics::ContextWrapper;
use crate::core::graphics::opengl::OpenGLContext;
use crate::events::event::EventCallbackFn;

pub(crate) struct LinuxWindow<'a> {
    props: WindowProps,
    callback: EventCallbackFn,
    vsync: u8,
    window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    context_wrapper: &'a mut (dyn ContextWrapper + 'a)
}

impl<'a> LinuxWindow<'a> {

    pub fn new(props: WindowProps, callback: EventCallbackFn, vsync: u8, window: glfw::Window, events: Receiver<(f64, glfw::WindowEvent)>) -> LinuxWindow<'a> {
        debug!("1 glfw reports context version is {}", window.get_context_version());
        unsafe {
            let y = OpenGLContext::new(window.glfw).as_mut().unwrap();
            LinuxWindow { props: props, callback: callback, vsync: vsync, window: window, event_receiver: events, context_wrapper: y }
        }
    }
}

impl<'a> WindowBehavior<'a> for LinuxWindow<'a> {

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

    fn get_vsync(&self) -> u8 {
        self.vsync
    }

    fn set_vsync(&mut self, interval: u8) {
        match interval {
            0 => {
                self.window.glfw.set_swap_interval(glfw::SwapInterval::None);
                self.vsync = 0;
            }
            1 => {
                self.window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
                self.vsync = 1;
            }
            2 => {
                self.window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
                self.vsync = 2;
            }
            _ => {
                self.window.glfw.set_swap_interval(glfw::SwapInterval::None);
                self.vsync = 0;
            }
        }
    }

    fn get_props(&self) -> & WindowProps {
        & self.props
    }

    fn get_native_window(&mut self) -> (Option<&mut glfw::Window>, Option<&mut glfw::Window>) {
        (Some(&mut self.window), None)
    }

    fn on_update(&mut self) -> bool {
        self.window.glfw.poll_events();
        self.window.swap_buffers();
        false
    }

    fn get_context_wrapper(&mut self) -> &mut dyn ContextWrapper {
       self.context_wrapper
    }
}
