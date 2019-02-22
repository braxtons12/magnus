/**User MUST declare #![no_main] in their main.rs
 * 
 * User MUST implement this function for a struct serving as their app
 * #[no_mangle]
 * pub fn create_application() -> impl Box<Application>;
 *
 **/

use crate::events::event::Event;
use crate::events::event::EventDispatcher;
use crate::events::application_event::WindowResizeEvent;
use crate::events::application_event::WindowClosedEvent;

pub struct MagnusApplication {
    name:   String,
    running: bool,
    window: core::window::Window,
    layer_stack: core::layer::LayerStack,
}

impl MagnusApplication {

    pub fn new(name: String, width: i32, height: i32) -> MagnusApplication {
        MagnusApplication { name: name, running: false, window: Window::new(width, height), layer_stack: LayerStack::new() }
    }

    pub fn run(&self) -> () {

        debug!("Application {} Started", self.name);
        loop {

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

    pub fn on_window_resize(&mut self, e: &mut WindowResizeEvent) -> bool {
        e.set_handled(true);
        false
    }

    pub fn on_window_close(&self, e: &mut WindowClosedEvent) -> bool {
        e.set_handled(true);
        self.set_running(false);
        true
    }

    pub fn on_event(&mut self, e: Box<Event>) {
        let mut dispatcher = EventDispatcher::new(e);
        dispatcher.dispatch(on_window_resize);
        dispatcher.dispatch(on_window_close);

        for()
    }

}

