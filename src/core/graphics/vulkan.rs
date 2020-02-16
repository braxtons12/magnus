use std::fmt;
use std::sync::Arc;

use vulkano::instance::{ Instance, PhysicalDevice };
use vulkano::device::{ Device, DeviceExtensions };
use vulkano::swapchain::Surface;

pub struct VulkanContext {
    glfw: glfw::Glfw,
    device_id: usize,
    instance: Arc<Instance>,
    device: Arc<Device>,
    surface: Arc<Surface<glfw::Window>>,
}

impl fmt::Debug for VulkanContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VulkanContext {{ TODO: Insert debug info here }}")
    }
}

impl VulkanContext {

    pub fn new(window: glfw::Window, instance: Arc<Instance>, id: usize) -> VulkanContext {
        debug!("vulkan instance references is: {}", Arc::strong_count(&Arc::clone(&instance)));
        debug!("vulkan extensions are : {:?}", instance.loaded_extensions());
        debug!("creating vulkan devices");

        let temp = instance.clone();
        let physical_device = PhysicalDevice::from_index(&temp, id).unwrap();
        let queue_family = physical_device.queue_families().find(|&q| q.supports_graphics()).expect("Couldn't find graphical queue family");
        VulkanContext {
            glfw: window.glfw,
            device_id: id,
            surface: vulkano_glfw::create_window_surface(Arc::clone(&instance), window).expect("Failed to create vulkan surface"),
            instance,
            device: Device::new(physical_device, &physical_device.supported_features(), &DeviceExtensions::supported_by_device(physical_device), [(queue_family, 0.5)].iter().cloned()).expect("Failed to create vulkan device").0
        }
    }

    pub fn get_surface(&mut self) -> Arc<Surface<glfw::Window>> {
        self.surface.clone()
    }

    pub fn get_glfw(&mut self) -> &mut glfw::Glfw {
        &mut self.glfw
    }
}

