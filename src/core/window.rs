use std::sync::mpsc::Receiver;

use glfw;

//use crate::core::application;
use crate::core::settings::GraphicsMode;
use crate::events::event::EventCallbackFn;
use crate::events::key_events::*;
use crate::events::mouse_events::*;
use crate::events::window_events::*;
use crate::core::graphics;
use crate::core::graphics::opengl::OpenGLContext;
use crate::core::graphics::vulkan::VulkanContext;
#[cfg(windows)]
use crate::core::graphics::directx::DirectXContext;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
    graphics_mode: GraphicsMode
}

impl WindowProps {
    pub fn new(title: String, size: Option<(u32, u32)>, graphics_mode: GraphicsMode) -> WindowProps {
        WindowProps {
            title,
            width: match size {
                Some((w,_h)) => w,
                None => 800
            },
            height: match size {
                Some((_w, h)) => h,
                None => 600
            },
            graphics_mode
        }
    }
}

#[repr(C)]
pub struct Window<T: graphics::context::ContextLimiter> {
    props: WindowProps,
    //   callback: EventCallbackFn,
    vsync: u8,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    context: graphics::context::Context<T>
}

static mut GLFW_S: Option<glfw::Glfw> = None;

impl Window<OpenGLContext> {
    pub fn new(props: WindowProps) -> Window<OpenGLContext> {
        use glfw::Context;

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
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        window.make_current();

        let context = graphics::context::Context::<OpenGLContext>::new(window);

        Window {
            props,
            //            callback: application::MagnusApplication::on_event,
            vsync: 0,
            event_receiver: events,
            context
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<OpenGLContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
        //        self.callback = func;
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();
        let mut should_close = false;
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let mut x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let mut x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let mut x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let mut x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let mut x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let mut x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    ////(self.callback)(&mut x);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let mut x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    ////(self.callback)(&mut x);
                }
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let mut x = WindowCloseEvent::new("Window Should Close".to_string());
                        ////(self.callback)(&mut x);
                        should_close = true;
                    }
                }
            }

        }
        self.context.swap_buffers();
        should_close
    }


}

impl Window<VulkanContext> {
    pub fn new(props: WindowProps, id: usize) -> Window<VulkanContext> {
        use vulkano::instance::Instance;

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
            debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatibility");
            GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        let ext = &vulkano_glfw::get_required_instance_extensions(&window.glfw)
            .expect("Error getting required vulkan instance extensions");
        let instance = Instance::new(None, ext, None).expect("Failed to create vulkan instance");

        Window {
            props,
            //        callback: application::MagnusApplication::on_event,
            vsync: 0,
            event_receiver: events,
            context: graphics::context::Context::<VulkanContext>::new(window, instance, id)
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<VulkanContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
        //   self.callback = func;
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();
        let mut should_close = false;
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let mut x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let mut x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let mut x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let mut x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let mut x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let mut x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let mut x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                }
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let mut x = WindowCloseEvent::new("Window Should Close".to_string());
                        //(self.callback)(&mut x);
                        should_close = true;
                    }
                }
            }

        }
        should_close
    }


}

#[cfg(windows)]
impl Window<DirectXContext> {
    pub fn new(props: WindowProps) -> Window<DirectXContext> {

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
            debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatibility");
            GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);

        let context = graphics::context::Context::<DirectXContext>::new(window);

        Window {
            props,
            //   callback: application::MagnusApplication::on_event,
            vsync: 0,
            event_receiver: events,
            context
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<DirectXContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
        //self.callback = func;
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();
        let mut should_close = false;
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let mut x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let mut x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let mut x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let mut x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let mut x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let mut x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let mut x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    //(self.callback)(&mut x);
                }
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let mut x = WindowCloseEvent::new("Window Should Close".to_string());
                        //(self.callback)(&mut x);
                        should_close = true;
                    }
                }
            }

        }
        should_close
    }


}

