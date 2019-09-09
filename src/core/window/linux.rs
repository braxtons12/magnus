use std::sync::Arc;

use vulkano::swapchain::Surface;

use crate::core::settings::GraphicsMode;
use crate::core::window::*;
use crate::core::graphics::ContextWrapper;
use crate::core::graphics::opengl::OpenGLContext;
use crate::core::graphics::vulkan::VulkanContext;
use crate::events::key_events::*;
use crate::events::mouse_events::*;
use crate::events::window_events::*;
use crate::events::event::EventCallbackFn;

pub(crate) struct LinuxWindow<'a> {
    props: WindowProps,
    callback: EventCallbackFn,
    vsync: u8,
    glfw_context: glfw::Glfw,
    glfw_render_context: glfw::RenderContext,
    vulkan_surface: Option<Arc<Surface<glfw::Window>>>,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    context_wrapper: &'a mut (dyn ContextWrapper<'a> + 'a)
}

impl<'a> LinuxWindow<'a> {

    pub fn new(props: WindowProps, callback: EventCallbackFn, vsync: u8, window: glfw::Window, 
               render_context: glfw::RenderContext, 
               events: Receiver<(f64, glfw::WindowEvent)>, mode: GraphicsMode, 
               vulkan_id: usize) -> LinuxWindow<'static> 
    {
        if mode == GraphicsMode::OpenGL {
            debug!("1 glfw reports context version is {}", window.get_context_version());
        }
        unsafe {
            let context = window.glfw;
            let mut surface: Option<Arc<Surface<glfw::Window>>> = None;
            let y: &mut dyn ContextWrapper = match mode {
                GraphicsMode::DirectX => panic!("Cannot use DirectX on Linux"),
                GraphicsMode::OpenGL  => Box::into_raw(OpenGLContext::new(window)).as_mut().unwrap(),
                GraphicsMode::Vulkan  => {
                    let z = Box::into_raw(VulkanContext::new(window.glfw, vulkan_id)).as_mut().unwrap();
                    surface = Some(z.create_window_surface(window).unwrap());
                    z
                }
            };
            LinuxWindow { props: props, callback: callback, vsync: vsync,
            glfw_context: context,
            glfw_render_context: render_context,
            vulkan_surface: surface,
            event_receiver: events, context_wrapper: y}
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
                self.glfw_context.set_swap_interval(glfw::SwapInterval::None);
                self.vsync = 0;
            }
            1 => {
                self.glfw_context.set_swap_interval(glfw::SwapInterval::Adaptive);
                self.vsync = 1;
            }
            2 => {
                self.glfw_context.set_swap_interval(glfw::SwapInterval::Sync(1));
                self.vsync = 2;
            }
            _ => {
                self.glfw_context.set_swap_interval(glfw::SwapInterval::None);
                self.vsync = 0;
            }
        }
    }

    fn get_props(&self) -> & WindowProps {
        & self.props
    }

    fn on_update(&mut self) -> bool {
        self.glfw_context.poll_events();
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
        self.glfw_render_context.swap_buffers();
        should_close
    }

    fn get_context_wrapper(&mut self) -> &mut dyn ContextWrapper<'a> {
       self.context_wrapper
    }
}
