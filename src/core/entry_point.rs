#[allow(improper_ctypes)]

use crate::core;
use crate::core::application::MagnusApplication;

#[no_mangle]
extern "Rust" {
    pub fn create_application() -> MagnusApplication<'static>;
}

#[cfg(any(unix, linux))]
#[no_mangle]
pub extern "C" fn main(argc: isize, argv : *const *const u8) -> isize {

    if core::setup_logger().is_err() {
        println!("Error, could not init loggers");
        return -1;
    }

    unsafe {
        let mut app = create_application();
        app.run();
    }

    0
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn start(argc: iszie, argv: *const *const u8) -> iszie {

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
