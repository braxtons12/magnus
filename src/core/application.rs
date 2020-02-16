/**User MUST declare #![no_main] in their main.rs
 *
 * User MUST implement this function for a struct serving as their app
 * #[no_mangle]
 * pub fn create_application() -> impl Box<Application>;
 *
 **/
use crate::events::event::*;
use crate::events::event::{Event, EventDispatcher};
use crate::core::settings::Settings;
use crate::core::settings::GraphicsMode;
use crate::core::graphics;
use crate::core::graphics::opengl::OpenGLContext;
use crate::core::graphics::vulkan::VulkanContext;
#[cfg(windows)]
use crate::core::graphics::directx::DirectXContext;
use crate::core::window::*;
use crate::core::layers::*;

#[repr(C)]
pub struct MagnusApplication<T: graphics::context::ContextLimiter> {
    name: String,
    running: bool,
    settings: Settings,
    window: Window<T>,
    layer_stack: LayerStack
}

impl MagnusApplication<OpenGLContext> {
    pub fn new(name: String, settings: Settings) -> MagnusApplication<OpenGLContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<OpenGLContext>::new(props),
            layer_stack: LayerStack::new(None, None) }
    }

    pub fn run(&mut self)  {

        debug!("Application {} Started", self.name);
        self.window.get_context().api_context().load_symbols().expect("Failed to load graphics context symbols");
        'main: loop {
            debug!("Window width is {}", self.window.get_width());
            debug!("Changing clear color");
            unsafe {
                if self.settings.graphics().mode() == GraphicsMode::OpenGL {
                    gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
            for item in self.layer_stack.iter_mut() {
                item.on_update();
            }
            if self.window.on_update() {
                break 'main;
            }
        }
    }
}

impl MagnusApplication<VulkanContext> {
    pub fn new(name: String, settings: Settings) -> MagnusApplication<VulkanContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<VulkanContext>::new(props, settings.graphics().vulkan_id()),
            layer_stack: LayerStack::new(None, None) }
    }

    pub fn run(&mut self)  {

        debug!("Application {} Started", self.name);
        'main: loop {
            debug!("Window width is {}", self.window.get_width());
            debug!("Changing clear color");
            unsafe {
                if self.settings.graphics().mode() == GraphicsMode::OpenGL {
                    gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
            for item in self.layer_stack.iter_mut() {
                item.on_update();
            }
            if self.window.on_update() {
                break 'main;
            }
        }
    }
}

#[cfg(windows)]
impl MagnusApplication<DirectXContext> {
    pub fn new(name: String, width: i32, height: i32, settings: Settings) -> MagnusApplication<DirectXContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<DirectXContext>::new(props),
            layer_stack: LayerStack::new(None, None) }
    }

    pub fn run(&mut self)  {

        debug!("Application {} Started", self.name);
        'main: loop {
            debug!("Window width is {}", self.window.get_width());
            debug!("Changing clear color");
            unsafe {
                if self.settings.graphics().mode() == GraphicsMode::OpenGL {
                    gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
            for item in self.layer_stack.iter_mut() {
                item.on_update();
            }
            if self.window.on_update() {
                break 'main;
            }
        }
    }
}

impl<T: graphics::context::ContextLimiter> MagnusApplication<T> {
    pub fn push_layer(&mut self, layer: Layer) {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, layer: Layer) {
        self.layer_stack.push_overlay(layer);
    }

    pub fn remove_layer(&mut self) -> Layer {
        self.layer_stack.remove_layer()
    }

    pub fn remove_overlay(&mut self) -> Layer {
        self.layer_stack.remove_overlay()
    }

    pub fn layer_stack(&mut self) -> &mut LayerStack {
        &mut self.layer_stack
    }

    #[inline(always)]
    pub fn get_running(&self) -> bool {
        self.running
    }

    #[inline(always)]
    pub fn set_running(&mut self, set: bool) {
        self.running = set;
    }

    //#[inline(always)]
    //fn get_layer_stack(&self) -> LayerStack

    pub fn on_window_resize(&mut self, e: &mut dyn Event) -> bool {
        let h = e.handled();
        *h = true;
        false
    }

    pub fn on_window_close(&mut self, e: &mut dyn Event) -> bool {
        let h = e.handled();
        *h = true;
        self.set_running(false);
        true
    }

    //pub fn on_event(e: &mut (dyn Event)) -> bool {

    //debug!("Processing event e: {}", e);
    //unsafe {
    //let app = APP.as_mut().unwrap();
    //'event: for item in app.layer_stack().iter_mut() {
    //item.on_event(e);
    //if * e.handled() {
    //break 'event
    //}
    //}
    //}
    //true
    //}
}
