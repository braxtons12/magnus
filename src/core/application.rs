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
use crate::core::window::*;

#[repr(C)]
pub struct MagnusApplication<'a> {
    name: String,
    running: bool,
    settings: Settings,
    window: Window<'a>
}

impl<'a> MagnusApplication<'a> {

    pub fn new(name: String, width: i32, height: i32) -> MagnusApplication<'a> {
        let mut sets = Settings::new(&name);
        let props = WindowProps::new(name.clone(), Some(width as u32), Some(height as u32), Some(sets.graphics().mode()));
        sets.set_graphics_mode(GraphicsMode::Vulkan);
        //sets.set_graphics_mode(GraphicsMode::DirectX);

        MagnusApplication { name, running: true, settings: sets, window: Window::new(props, sets)}
    }

    pub fn run(&mut self)  {

        debug!("Application {} Started", self.name);
        if self.settings.graphics().mode() == GraphicsMode::OpenGL {
            self.window.get_context().load_symbols().expect("Failed to load graphics context symbols");
        }
        if self.settings.graphics().mode() == GraphicsMode::Vulkan {
            self.window.create_vulkan_devices().expect("Failed to create vulkan devices");
        }
        'main: loop {
                debug!("Window width is {}", self.window.get_width());
                debug!("Changing clear color");
                unsafe {
                    if self.settings.graphics().mode() == GraphicsMode::OpenGL {
                        gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                }
                if self.window.on_update() {
                    break 'main;
                }
        }
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

    pub fn on_event(e:& mut (dyn Event)) -> bool {

        debug!("Processing event e: {}", e);
        true
    }
}
