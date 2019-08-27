use crate::events::event::EventCallbackFn;
use win32::*;
use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

pub(crate) mod win32;
//will add support later
//pub(crate) mod linux;
//pub(crate) mod osx;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32
}

impl WindowProps {
    pub fn new(title: String, width: Option<u32>, height: Option<u32>) -> WindowProps {
        WindowProps { title: title, width: width.unwrap_or(1280), height: height.unwrap_or(720) }
    }
}

pub struct Window<'a> {
    window: Box<dyn WindowBehavior + 'a>,
    glfw_events: Receiver<(f64, glfw::WindowEvent)>
}

impl<'a> Window<'a> {
    pub fn new(props: WindowProps) -> Window<'a>{
        let (win, ev) = create(props).unwrap();
        Window { window: win, glfw_events: ev }
    }

    pub fn get_window(&self) -> & Box<dyn WindowBehavior + 'a> {
        &(self.window)
    }

    pub fn get_glfw_events(&self) -> & Receiver<(f64, glfw::WindowEvent)> {
        &(self.glfw_events)
    }
}

pub trait WindowBehavior {

    fn get_width(&self) -> u32;
    fn set_width(&mut self, w: u32);
    fn get_height(&self) -> u32;
    fn set_height(&mut self, h: u32);
    fn set_event_callback(&mut self, func: EventCallbackFn);
    fn is_vsync(&self) -> bool;
    fn set_vsync(&mut self, enabled: bool);
    fn get_glfw_window(&self) -> & glfw::Window;
}

fn create<'a>(props: WindowProps) -> Option<(Box<dyn WindowBehavior + 'a>, Receiver<(f64, glfw::WindowEvent)>)> {
        if cfg!(windows) {
            let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

            debug!("Creating Windows Window: {}", props.title);

            let (mut window, events) = glfw.create_window(props.width, props.height, props.title.as_str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
            window.set_key_polling(true);
            window.make_current();

            return Some((Win32Window::new(props, Win32Window::on_update, false, window), events));
        } else {
            return None;
        }
}
