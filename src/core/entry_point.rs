#[allow(improper_ctypes)]

use crate::core::application::MagnusApplication;

#[no_mangle]
extern "Rust" {
    pub fn create_application() -> MagnusApplication<'static>;
}

#[macro_export]
macro_rules! magnus_engine {
    () => {
        pub fn main() {
            use magnus::core::setup_logger;

            if setup_logger().is_err() {
                panic!("Error, could not init loggers!");
            }

            unsafe {
                let mut app = create_application();
                app.run();
            }
        }
    }
}
