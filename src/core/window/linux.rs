use crate::core::window::*;
use crate::core::graphics::ContextWrapper;
use crate::core::graphics::opengl::OpenGLContext;
use crate::events::key_events::*;
use crate::events::mouse_events::*;
use crate::events::window_events::*;
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

    pub fn callback(&self, e: &mut (dyn Event)) -> bool {
        (self.callback)(e)
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
        let mut should_close = false;
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let mut x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    self.callback(&mut x);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let mut x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    self.callback(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let mut x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    self.callback(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let mut x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    self.callback(&mut x);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    self.callback(&mut x);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    self.callback(&mut x);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let mut x = WindowFocusEvent::new(format!("Window Focused"), focus);
                    self.callback(&mut x);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let mut x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    self.callback(&mut x);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let mut x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    self.callback(&mut x);
                }
                _ => { 
                    if event == glfw::WindowEvent::Close {
                        let mut x = WindowCloseEvent::new(format!("Window Should Close"));
                        self.callback(&mut x);
                        should_close = true;
                    }
                }
            }
            
        }
        self.window.swap_buffers();
        should_close
    }

    fn get_context_wrapper(&mut self) -> &mut dyn ContextWrapper {
       self.context_wrapper
    }
}
