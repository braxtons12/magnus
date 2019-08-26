
#[allow(improper_ctypes)]

use crate::core;
use crate::core::application::MagnusApplication;

#[no_mangle]
extern "Rust" {
    pub fn create_application() -> MagnusApplication;
}

#[no_mangle]
pub fn main(_argc: isize, _argv : *const *const u8) -> isize {

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
