
use crate::core;
use crate::core::application;

use std::boxed::Box;

#[no_mangle]
extern "Rust" {
    pub fn create_application() -> Box<application::Application>;
}

#[no_mangle]
pub fn main(argc: isize, argv : *const *const u8) -> isize {

    if core::setup_logger().is_err() {
        println!("Error, could not init loggers");
        return -1;
    }

    unsafe {
        let app = create_application();
        app.run();
    }

    0
} 
