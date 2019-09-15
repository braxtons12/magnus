use crate::core::graphics::DeviceCreationError;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use glfw;
use glfw::Context;

use vulkano::swapchain::Surface;

use crate::core::application;
use crate::core::settings::{ Settings, GraphicsMode };
use crate::events::event::{ Event, EventCallbackFn };
use crate::events::key_events::*;
use crate::events::mouse_events::*;
use crate::events::window_events::*;
use crate::core::graphics;
use crate::core::graphics::{ ContextWrapper, opengl::OpenGLContext,
directx::DirectXContext, vulkan::VulkanContext };

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
	graphics_mode: GraphicsMode
}

impl WindowProps {
    pub fn new(title: String, width: Option<u32>, height: Option<u32>, mode: Option<GraphicsMode>) -> WindowProps {
        WindowProps { title, width: width.unwrap_or(1280), height: height.unwrap_or(720), graphics_mode: mode.unwrap_or(GraphicsMode::OpenGL) }
    }
}

#[repr(C)]
pub struct Window<'a> {
	props: WindowProps,
	callback: EventCallbackFn,
	vsync: u8,
	gl: glfw::Glfw,
	glfw_render_context: Option<glfw::RenderContext>,
	glfw_w: Option<glfw::Window>,
	event_receiver: Receiver<(f64, glfw::WindowEvent)>,
	vulkan_surface: Option<Arc<Surface<glfw::Window>>>,
    context: graphics::Context<'a>
}

static mut GLFW_S: Option<glfw::Glfw> = None;

#[cfg(windows)]
impl<'a> Window<'a> {
    pub fn new(props: WindowProps, settings: Settings) -> Window<'a> {
		unsafe {
			if GLFW_S.is_none() {
				GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
			}
		}

		let mode = settings.graphics().mode();

		debug!("Creating Window: {}", props.title);
		let mut window: glfw::Window;
		let events: Reciever<(f64, glfw::WindowEvent)>;
		let x: (glfw::Window, Reciever<(f64, glfw::WindowEvent)>);
		unsafe {
			if mode != GraphicsMode::OpenGL {
				debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatibility");
				GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
			}
			debug!("Creating glfw window");
			x = GFLW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
			glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
		}

		window = x.0;
		events = x.1;
		window.set_all_polling(true);
		if mode == GraphicsMode::OpenGL {
			window.make_current();
		}
		let mut surface: Option<Arc<Surface<glfw::Window>>> = None;
		let render_context = window.render_context();
		let mut win: Option<glfw::Window> = None;
		let gl = window.glfw;
		let wrapper: Box<dyn ContextWrapper<'a> + 'a> = match mode {
			GraphicsMOde::OpenGL => {
				let wrap = OpenGLContext::new(window.glfw);
				win = Some(window);
				Box::from(wrap) as Box<dyn ContextWrapper<'a> + 'a>
			},
			GraphicsMode::DirectX => {
				let (wrap, w) = DirectXContext::new(window);
				win = Some(w);
				Box::from(wrap) as Box<dyn ContextWrapper<'a> + 'a>
			},
			GraphicsMode::Vulkan => {
				let mut wrap: VulkanContext<'a> = VulkanContext::new(window.glfw, settings.graphics().vulkan_id());
				surface = Some(wrap.create_window_surface(window).expect("Failed to create window surface"));
				Box::from(wrap.clone()) as Box<dyn ContextWrapper<'a> + 'a>
			}
		};

		Window {
			props,
			callback: application::MagnusApplication::on_event,
			vsync: 0,
			gl,
			glfw_render_context: match mode {
				GraphicsMode::OpenGL => Some(render_context),
				_					 => None
			},
			glfw_w: win,
			event_receiver: events,
			vulkan_surface: surface,
			context: graphics::Context::new(wrapper)
		}
    }
}

#[cfg(unix)]
impl<'a> Window<'a> {
    pub fn new(props: WindowProps, settings: Settings) -> Window<'a> {
		unsafe {
			if GLFW_S.is_none() {
				GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
			}
		}

		let mode = settings.graphics().mode();

		debug!("Creating Window: {}", props.title);
		let mut window: glfw::Window;
		let events: Receiver<(f64, glfw::WindowEvent)>;
		let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
		unsafe {
			if mode != GraphicsMode::OpenGL {
				debug!("Setting ClientAPI WindowHint to NoApi for vulkan compatability");
				GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
			}
			debug!("Creating glfw window");
			x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
			glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
		}

		window = x.0;
		events = x.1;
		window.set_all_polling(true);
		if mode == GraphicsMode::OpenGL {
			window.make_current();
		}
		let mut surface: Option<Arc<Surface<glfw::Window>>> = None;
		let render_context = window.render_context();
		let mut win: Option<glfw::Window> = None;
		let gl = window.glfw;
		let wrapper: Box<dyn ContextWrapper<'a> + 'a> = match mode {
			GraphicsMode::OpenGL => {
				let wrap = OpenGLContext::new(window.glfw);
				win = Some(window);
				Box::from(wrap) as Box<dyn ContextWrapper<'a> + 'a>
			},
			GraphicsMode::DirectX => panic!("Can't use DirectX on Linux!"),
			GraphicsMode::Vulkan  => {
				let mut wrap = VulkanContext::new(window.glfw, settings.graphics().vulkan_id());
				surface = Some(wrap.create_window_surface(window).expect("Failed to create window surface"));
				Box::from(wrap) as Box<dyn ContextWrapper<'a> + 'a>
			}
		};

		Window {
			props,
			callback: application::MagnusApplication::on_event,
			vsync: 0,
			gl,
			glfw_render_context: match mode {
				GraphicsMode::OpenGL => Some(render_context),
				_ 					 => None
			},
			glfw_w: win,
			event_receiver: events,
			vulkan_surface: surface,
			context: graphics::Context::new(wrapper)
		}
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
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
		self.props.width = w;
    }

    pub fn get_height(&self) -> u32 {
		self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
		self.props.height = h;
    }

    pub fn set_event_callback(&mut self, func: EventCallbackFn) {
		self.callback = func;
    }

    pub fn get_vsync(&self) -> u8 {
		self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
		match interval {
			0 => {
				self.gl.set_swap_interval(glfw::SwapInterval::None);
				self.vsync = 0;
			},
			1 => {
				self.gl.set_swap_interval(glfw::SwapInterval::Adaptive);
				self.vsync = 1;
			},
			2 => {
				self.gl.set_swap_interval(glfw::SwapInterval::Sync(1));
				self.vsync = 2;
			},
			_ => {
				self.gl.set_swap_interval(glfw::SwapInterval::None);
				self.vsync = 0;
			}
		}
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn on_update(&mut self) -> bool {
        self.gl.poll_events();
        let mut should_close = false;
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let mut x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let mut x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let mut x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let mut x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let mut x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let mut x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let mut x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    (self.callback)(&mut x);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let mut x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    (self.callback)(&mut x);
                }
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let mut x = WindowCloseEvent::new("Window Should Close".to_string());
                        (self.callback)(&mut x);
                        should_close = true;
                    }
                }
            }

        }
        if self.glfw_render_context.is_some() {
            self.glfw_render_context.as_mut().unwrap().swap_buffers();
        }
        should_close
	}

    pub fn get_context(&mut self) -> &mut graphics::Context<'a> {
        &mut self.context
    }

    pub fn create_vulkan_devices(& mut self) -> Result<(), DeviceCreationError> {
        self.context.create_devices()
    }
}
