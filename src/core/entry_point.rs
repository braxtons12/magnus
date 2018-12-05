
use core::application;
use std::boxed::Box;

#[no_mangle]
extern "Rust" {
    pub fn create_application() -> Box<application::Application>;
}

#[no_mangle]
pub fn main(argc: isize, argv : *const *const u8) -> isize {

    unsafe {
        let app = create_application();
        app.run();
    }

    0
} 
