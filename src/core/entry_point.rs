#![allow(improper_ctypes)]
use crate::core::settings::Settings;

#[no_mangle]
extern "Rust" {
    pub fn prelude() -> (Settings, String);
}

#[macro_export]
#[cfg(not(windows))]
macro_rules! magnus_engine {
    () => {
        pub fn main() {
            use magnus::core::setup_logger;
            use magnus::core::application::MagnusApplication;
            use magnus::core::graphics::opengl::OpenGLContext;
            use magnus::core::graphics::vulkan::VulkanContext;
            use magnus::core::graphics::context::ContextLimiter;

            if setup_logger().is_err() {
                panic!("Error, could not init loggers!");
            }

            let (settings, app_name) = prelude();
            match settings.graphics().mode() {
                GraphicsMode::OpenGL => MagnusApplication::<OpenGLContext>::new(app_name, settings).run(),
                GraphicsMode::Vulkan => MagnusApplication::<VulkanContext>::new(app_name, settings).run(),
                _ => panic!("Not running on windows")
            };
        }
    }
}

#[macro_export]
#[cfg(windows)]
macro_rules! magnus_engine {
    () => {
        pub fn main() {
            use magnus::core::setup_logger;
            use magnus::core::application::MagnusApplication;
            use magnus::core::graphics::opengl::OpenGLContext;
            use magnus::core::graphics::vulkan::VulkanContext;
            use magnus::core::graphics::directx::DirectXContext;
            use magnus::core::graphics::context::ContextLimiter;

            if setup_logger().is_err() {
                panic!("Error, could not init loggers!");
            }

            let (settings, app_name) = prelude();
            match settings.graphics().mode() {
                GraphicsMode::OpenGL => MagnusApplication::<OpenGLContext>::new(app_name, settings).run(),
                GraphicsMode::Vulkan => MagnusApplication::<VulkanContext>::new(app_name, settings).run(),
                GraphicsMode::DirectX => MagnusApplication::<DirectXContext>::new(app_name, settings).run()
            };
        }
    }
}
