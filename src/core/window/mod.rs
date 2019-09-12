use crate::core::graphics::DeviceCreationError;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use glfw;
use glfw::Context;

use vulkano::swapchain::Surface;

use crate::core::application;
use crate::core::settings::{ Settings, GraphicsMode };
use crate::events::event::{ Event, EventCallbackFn };
use crate::core::graphics;
use crate::core::graphics::{ ContextWrapper, opengl::OpenGLContext,
                            directx::DirectXContext, vulkan::VulkanContext };
use win32::Win32Window;
use linux::LinuxWindow;

pub(crate) mod win32;
pub(crate) mod linux;
//will add support later
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
    window: Box<dyn WindowBehavior<'a>>,
    context: graphics::Context<'a>
}

static mut GLFW_S: Option<glfw::Glfw> = None;

#[cfg(windows)]
impl<'a, 'b> Window<'a> {
    pub fn new(props: WindowProps, settings: Settings) -> Window<'a> {
        let (window, mut context) = Window::create(props, settings);
        debug!("Success creating window");

        Window { window, context }
    }

    fn create(props: WindowProps, settings: Settings) -> (Box<dyn WindowBehavior<'b>>, graphics::Context<'b>) {
        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
            }
        }

        debug!("Creating Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            if settings.graphics().mode() != GraphicsMode::OpenGL {
                debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatability");
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
            window.make_current();
        }
        let mut surface: Option<Arc<Surface<glfw::Window>>> = None;
        let render_context = window.render_context();
        let mut win: Option<glfw::Window> = None;
        let gl = window.glfw;
        let wrapper: Box<dyn ContextWrapper<'b> + 'b> = match settings.graphics().mode() {
            GraphicsMode::OpenGL  => {
                let wrap = OpenGLContext::new(window.glfw);
                win = Some(window);
                Box::from(wrap) as Box<dyn ContextWrapper<'b> + 'b>
            },
            GraphicsMode::DirectX => {
                let (wrap, w) = DirectXContext::new(window);
                win = Some(w);
                Box::from(wrap) as Box<dyn ContextWrapper<'b> + 'b>

            },
            GraphicsMode::Vulkan  => {
                let mut wrap: VulkanContext<'b> = VulkanContext::new(window.glfw, settings.graphics().vulkan_id());
                surface = Some(wrap.create_window_surface(window).expect("Failed to create window surface"));
                Box::from(wrap.clone()) as Box<dyn ContextWrapper<'b> + 'b>
            }
        };

        (Box::from(Win32Window::new(props, application::MagnusApplication::on_event, 0, gl, win,
                          match settings.graphics().mode() {
                GraphicsMode::OpenGL => Some(render_context),
                _                    => None
            },
        events, surface)), graphics::Context::new(wrapper))
    }
}

#[cfg(unix)]
impl<'a, 'b> Window<'a> {
    pub fn new(props: WindowProps, settings: Settings) -> Window<'a> {
        let (window, context) = Window::create(props, settings);
        debug!("Success creating window: {}!", props.title);

        Window{ window, context }
    }

    fn create(props: WindowProps, settings: Settings) -> (Box<dyn WindowBehavior<'b>>, graphics::Context<'b>) {
        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
            }
        }

        debug!("Creating Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            if settings.graphics().mode() != GraphicsMode::OpenGL {
                debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatability");
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
            window.make_current();
        }
        let mut surface: Option<Arc<Surface<glfw::Window>>> = None;
        let render_context = window.render_context();
        let mut win: Option<glfw::Window> = None;
        let gl = window.glfw;
        let wrapper: Box<dyn ContextWrapper<'b> + 'b> = match settings.graphics().mode() {
            GraphicsMode::OpenGL  => {
                let wrap = OpenGLContext::new(window.glfw);
                win = Some(window);
                Box::from(wrap) as Box<dyn ContextWrapper<'b> + 'b>
            },
            GraphicsMode::DirectX => panic!("Can't use DirectX on Linux!"),
            GraphicsMode::Vulkan  => {
                let mut wrap = VulkanContext::new(window.glfw, settings.graphics().vulkan_id());
                surface = Some(wrap.create_window_surface(window).expect("Failed to create window surface"));
                Box::from(wrap) as Box<dyn ContextWrapper<'b> + 'b>
            }
        };

        (Box::from(LinuxWindow::new(props, application::MagnusApplication::on_event, 0, gl, win,
                          match settings.graphics().mode() {
                              GraphicsMode::OpenGL => Some(render_context),
                              _                    => None
                          },
                          events, surface)), wrapper)
    }
}


impl<'a> Window<'a> {

    /** don't think we need this anymore, but will leave it until I'm sure
      pub fn get_window<'a>(&self) -> &'a dyn WindowBehavior {
      unsafe {
      self.window.as_ref().unwrap()
      }
      }
     **/

    pub fn get_width(&self) -> u32 {
        self.window.get_width()
    }

    pub fn set_width(&mut self, w: u32) {
        self.window.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.window.get_height()
    }

    pub fn set_height(&mut self, h: u32) {
        self.window.set_height(h);
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
        self.window.set_event_callback(func);
    }

    pub fn get_vsync(&self) -> u8 {
        self.window.get_vsync()
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.window.set_vsync(interval);
    }

    pub fn get_props(&self) -> & WindowProps {
        self.window.get_props()
    }

    pub fn on_update(&mut self) -> bool {
        self.window.on_update()
    }

    pub fn get_context(&mut self) -> &mut graphics::Context<'a> {
        &mut self.context
    }

    pub fn create_vulkan_devices(& mut self) -> Result<(), DeviceCreationError> {
        self.context.create_devices()
    }
}

pub(crate) trait WindowBehavior<'a> {

    fn get_width(&self) -> u32;
    fn set_width(&mut self, w: u32);
    fn get_height(&self) -> u32;
    fn set_height(&mut self, h: u32);    fn set_event_callback(&mut self, func: EventCallbackFn);
    fn get_vsync(&self) -> u8;
    fn set_vsync(&mut self, interval: u8);
    fn get_props(&self) -> & WindowProps;
    fn on_update(&mut self) -> bool;
}
