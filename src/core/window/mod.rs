use crate::core::application;
use crate::events::event::{Event, EventCallbackFn};
use crate::core::graphics;
use win32::*;
use linux::*;

use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

pub(crate) mod win32;
pub(crate) mod linux;
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

#[repr(C)]
pub struct Window<'a> {
    window: *mut dyn WindowBehavior<'a>,
    event_receiver: (Option<Receiver<(f64, glfw::WindowEvent)>>, Option<Receiver<(f64, glfw::WindowEvent)>>),
    context: graphics::Context<'a>
        //when win32 window updated for alt wrapper (sdl??)/ or raw win32, update to match
}

impl<'a> Window<'a> {
    pub fn new(props: WindowProps) -> Window<'static> {
        let (mut win, ev) = create(props).expect("Error Creating Window");
        debug!("Success creating window: {}!", win.get_props().title);
        let (nwin, _not) = win.get_native_window();
        debug!("3. glfw reports context version is {}", nwin.unwrap().get_context_version());
        let x = Box::into_raw(win);
        let y: &dyn WindowBehavior; 
        unsafe {
            let y = x.as_mut().unwrap();
            Window { window: x, event_receiver: ev, context: graphics::Context::new(y.get_context_wrapper()) }
        }
    }

    /** don't think we need this anymore, but will leave it until I'm sure
      pub fn get_window<'a>(&self) -> &'a dyn WindowBehavior {
      unsafe {
      self.window.as_ref().unwrap()
      }
      }
     **/

    pub fn get_event_receiver(&self) -> & (Option<Receiver<(f64, glfw::WindowEvent)>>, Option<Receiver<(f64, glfw::WindowEvent)>>) {
        & self.event_receiver
    }

    pub fn get_width(&self) -> u32 {
        unsafe {
            self.window.as_ref().unwrap().get_width()
        }
    }

    pub fn set_width(&mut self, w: u32) {
        unsafe {
            self.window.as_mut().unwrap().set_width(w);
        }
    }

    pub fn get_height(&self) -> u32 {
        unsafe {
            self.window.as_ref().unwrap().get_height()
        }
    }

    pub fn set_height(&mut self, h: u32) {
        unsafe {
            self.window.as_mut().unwrap().set_height(h);
        }
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
        unsafe {
            self.window.as_mut().unwrap().set_event_callback(func);
        }
    }

    pub fn get_vsync(&self) -> u8 {
        unsafe {
            self.window.as_ref().unwrap().get_vsync()
        }
    }

    pub fn set_vsync(&mut self, interval: u8) {
        unsafe {
            self.window.as_mut().unwrap().set_vsync(interval);
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        unsafe {
            self.window.as_ref().unwrap().get_props()
        }
    }

    //when win32 window udpated for alt wrapper (sdl??) or raw win32, update to match
    pub fn get_native_window(&mut self) -> (Option<&mut glfw::Window>, Option<&mut glfw::Window>) {
        unsafe {
            self.window.as_mut().unwrap().get_native_window()
        }
    }

    pub fn on_update(&mut self) {
        unsafe {
            self.window.as_mut().unwrap().on_update();
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::Context<'a> {
        &mut self.context
    }
}

pub(crate) trait WindowBehavior<'a> {

    fn get_width(&self) -> u32;
    fn set_width(&mut self, w: u32);
    fn get_height(&self) -> u32;
    fn set_height(&mut self, h: u32);
    fn set_event_callback(&mut self, func: EventCallbackFn);
    fn get_vsync(&self) -> u8;
    fn set_vsync(&mut self, interval: u8);
    fn get_props(&self) -> & WindowProps;
    fn get_native_window(&mut self) -> (Option<&mut glfw::Window>, Option<&mut glfw::Window>); //when win32 window updated for alt wrapper (sdl??)/ or raw win32, update to match
    fn on_update(&mut self);
    fn get_context_wrapper(&mut self) -> &mut dyn graphics::ContextWrapper;
}

fn create<'a>(props: WindowProps) -> Option<(Box<dyn WindowBehavior<'static>>, (Option<Receiver<(f64, glfw::WindowEvent)>>, Option<Receiver<(f64, glfw::WindowEvent)>>))> {
    if cfg!(windows) {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        debug!("Creating Windows Window: {}", props.title);
        let (mut window, events) = glfw.create_window(props.width, props.height, props.title.as_str(), glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();

        return Some((Box::from(Win32Window::new(props, application::on_event, 0, window)), (Some(events), None)));
    } else if cfg!(unix) {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        debug!("Creating Linux Window: {}", props.title);

        let (mut window, events) = glfw.create_window(props.width, props.height, props.title.as_str(), glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();
        
        return Some((Box::from(LinuxWindow::new(props, application::on_event, 0, window)), (Some(events), None)));
    } else {
        return None;
    }
}
