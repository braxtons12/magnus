use std::sync::Arc;

use vulkano::instance::Instance;

use crate::core::graphics::opengl::OpenGLContext;
#[cfg(windows)]
use crate::core::graphics::directx::DirectXContext;
use crate::core::graphics::vulkan::VulkanContext;

pub trait ContextLimiter: Send {}

impl ContextLimiter for OpenGLContext {}
impl<'a> ContextLimiter for VulkanContext {}
#[cfg(windows)]
impl ContextLimiter for DirectXContext {}

#[cfg(not(windows))]
#[derive(Debug)]
pub struct Context<T: ContextLimiter> {
    api_context: T,
}

impl Context<OpenGLContext> {
    pub fn new(window: glfw::Window) -> Context<OpenGLContext> {
        Context { api_context: OpenGLContext::new(window) }
    }

    pub fn set_width(&mut self, w: u32) {
        let window = self.api_context.get_window();
        window.set_size(w as i32, window.get_size().1);
    }

    pub fn set_height(&mut self, h: u32) {
        let window = self.api_context.get_window();
        window.set_size(window.get_size().0, h as i32);
    }

    pub fn set_vsync(&mut self, interval: u8) {
        match interval {
            0 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::None),
            1 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::Adaptive),
            2 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::Sync(1)),
            _ => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::None)
        }
    }

    pub fn poll_events(&mut self) {
        self.api_context.get_window().glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        use glfw::Context;
        self.api_context.get_window().swap_buffers();
    }

    pub fn api_context(&mut self) -> &mut OpenGLContext {
        &mut self.api_context
    }
}

impl Context<VulkanContext> {
    pub fn new(window: glfw::Window, instance: Arc<Instance>, id: usize) -> Context<VulkanContext> {
        Context {
            api_context: VulkanContext::new(
                             window,
                             instance,
                             id)
        }
    }

    pub fn set_width(&mut self, w: u32) {
        //TODO: figure out how to update screen width in vulkan
        self.api_context.get_surface();
    }

    pub fn set_height(&mut self, h: u32) {
        //TODO: figure out how to update screen height in vulkan
    }

    pub fn set_vsync(&mut self, interval: u8) {
        //TODO: figure out how to update swap interval in vulkan
    }

    pub fn poll_events(&mut self) {
        self.api_context.get_glfw().poll_events();
    }

    pub fn api_context(&mut self) -> &mut VulkanContext {
        &mut self.api_context
    }
}

#[cfg(windows)]
impl Context<DirectXContext> {
    pub fn new(window: glfw::Window) -> Context<DirectXContext> {
        Context { api_context: DirectXContext::new(window) }
    }

    pub fn set_width(&mut self, w: u32) {
        let window = self.api_context.get_window();
        window.set_size(w as i32, window.get_size().1);
    }

    pub fn set_height(&mut self, h: u32) {
        let window = self.api_context.get_window();
        window.set_size(window.get_size().0, h as i32);
    }

    pub fn set_vsync(&mut self, interval: u8) {
        match interval {
            0 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::None),
            1 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::Adaptive),
            2 => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::Sync(1)),
            _ => self.api_context.get_window().glfw.set_swap_interval(glfw::SwapInterval::None)
        }
    }

    pub fn poll_events(&mut self) {
        self.api_context.get_window().glfw.poll_events();
    }

    pub fn api_context(&mut self) -> &mut DirectXContext {
        &mut self.api_context
    }
}

