/**User MUST declare #![no_main] in their main.rs
 * 
 * User MUST implement this function for a struct serving as their app
 * #[no_mangle]
 * pub fn create_application() -> impl Box<Application>;
 *
 **/
use crate::events::event::*;
use crate::events::event::Event;
use crate::events::event::EventDispatcher;

#[repr(C)]
pub struct MagnusApplication {
    name:   String,
    running: bool
}

impl MagnusApplication {

    pub fn new(name: String, _width: i32, _height: i32) -> MagnusApplication {
        MagnusApplication { name: name, running: false }
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

    //pub fn on_event(&mut self, e: Event) {
    //    let mut dispatcher = EventDispatcher::new(e);
    //    dispatcher.dispatch(MagnusApplication::on_window_resize);
    //    dispatcher.dispatch(on_window_close);
//
  //  }
}

