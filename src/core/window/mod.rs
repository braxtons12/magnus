use crate::core::graphics::DeviceCreationError;
use std::sync::mpsc::Receiver;

use glfw;
use glfw::Context;

use crate::core::application;
use crate::core::settings::Settings;
use crate::core::settings::GraphicsMode;
use crate::events::event::{Event, EventCallbackFn};
use crate::core::graphics;
use win32::*;
use linux::*;

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
        WindowProps { title, width: width.unwrap_or(1280), height: height.unwrap_or(720) }
    }
}

#[repr(C)]
pub struct Window<'a> {
    window: *mut dyn WindowBehavior<'a>,
    context: graphics::Context<'a>
        //when win32 window updated for alt wrapper (sdl??)/ or raw win32, update to match
}

impl<'a> Window<'a> {
    pub fn new(props: WindowProps, settings: Settings) -> Window<'static> {
        let win = create(props, settings).expect("Error Creating Window");
        debug!("Success creating window: {}!", win.get_props().title);

        let x = Box::into_raw(win);
        unsafe {
            let y = x.as_mut().unwrap();
            Window { window: x, context: graphics::Context::new(y.get_context_wrapper()) }
        }
    }

    /** don't think we need this anymore, but will leave it until I'm sure
      pub fn get_window<'a>(&self) -> &'a dyn WindowBehavior {
      unsafe {
      self.window.as_ref().unwrap()
      }
      }
     **/

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

    pub fn on_update(&mut self) -> bool {
        unsafe {
            self.window.as_mut().unwrap().on_update()
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::Context<'a> {
        &mut self.context
    }

    pub fn create_vulkan_devices(&mut self) -> Result<(), DeviceCreationError> {
        unsafe {
            self.window.as_mut().unwrap().get_context_wrapper().create_devices()
        }
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
    fn on_update(&mut self) -> bool;
    fn get_context_wrapper(&mut self) -> &mut dyn graphics::ContextWrapper<'a>;
}

static mut GLFW_S: Option<glfw::Glfw> = None;

fn create(props: WindowProps, settings: Settings) -> Option<Box<dyn WindowBehavior<'static>>> {
    if cfg!(windows) {
        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
            }
        }

        debug!("Creating Windows Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            if settings.graphics().mode() == GraphicsMode::Vulkan || settings.graphics().mode() == GraphicsMode::DirectX {
                debug!("Settings ClientApi WindowHint to NoApi for vulkan compatibility");
                GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            }
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
        }
        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        if settings.graphics().mode() == GraphicsMode::OpenGL {
            debug!("Making window the current OpenGL context");
            window.make_current();
        }
        
        debug!("Getting window RenderContext");
        let render_context = window.render_context();
        debug!("Calling contructor for Win32Window");
        Some(Box::from(Win32Window::new(props, application::MagnusApplication::on_event, 0, window, render_context, events, settings.graphics().mode(), settings.graphics().vulkan_id())))
    } else if cfg!(unix) {
        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
            }
        }

        debug!("Creating Windows Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            if settings.graphics().mode() == GraphicsMode::Vulkan {
                GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            }
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
        }
        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        if settings.graphics().mode() == GraphicsMode::OpenGL {
            window.make_current();
        }
        
        let render_context = window.render_context();
        Some(Box::from(LinuxWindow::new(props, application::MagnusApplication::on_event, 0, window, render_context, events, settings.graphics().mode(), settings.graphics().vulkan_id())))
    } else {
        None
    }
}
