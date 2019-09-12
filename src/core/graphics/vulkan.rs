use std::sync::Arc;
use std::any::Any;

use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::swapchain::Surface;
use vulkano_glfw;

use crate::core::graphics::{ContextWrapper, SymbolLoadError, DeviceCreationError};

#[derive(Clone)]
pub struct VulkanContext<'a> {
    gl: glfw::Glfw,
    vulkan_device_id: usize,
    vulkan_instance: Arc<Instance>,
    vulkan_physical_device: Option<PhysicalDevice<'a>>,
    vulkan_device: Option<Arc<Device>>,
    vulkan_queue_familes: Option<vulkano::instance::QueueFamily<'a>>
}

impl <'a> VulkanContext<'a> {
    pub fn new(gl: glfw::Glfw, id: usize) -> VulkanContext<'a> {
        let ext = &vulkano_glfw::get_required_instance_extensions(& gl)
        .expect("Error getting required vulkan instance extensions");

        let inst = Instance::new(None, ext, None).expect("Could not create vulkan instance");
        debug!("vulkan instance references is: {}", Arc::strong_count(&inst));

        VulkanContext { gl, vulkan_device_id: id, vulkan_instance: inst, 
        vulkan_physical_device: None, vulkan_device: None, vulkan_queue_familes: None }
    }
    
    pub fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        debug!("Creating vulkan window surface");
        debug!("Cloning vulkan instance");
        debug!("vulkan extensions are: {:?}", self.vulkan_instance.loaded_extensions());
        debug!("vulkan instace references is: {}", Arc::strong_count(&self.vulkan_instance));
        let x = Arc::clone(&self.vulkan_instance);
        debug!("Creating the surface via cloned instance");
        Some(vulkano_glfw::create_window_surface(x, glfw_w)
        .expect("Could not create vulkan window surface"))
    }

    pub fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        debug!("Creating Vulkan devices");
        let physical_device = PhysicalDevice::from_index(&self.vulkan_instance, self.vulkan_device_id).unwrap();
        let queue_family = physical_device.queue_families()
            .find(|&q| q.supports_graphics())
            .expect("Couldn't find a graphical queue family");
        
        let (device, _queues) = {
            Device::new(physical_device, &physical_device.supported_features(), &DeviceExtensions::supported_by_device(physical_device),
                [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device")
        };

        self.vulkan_physical_device = Some(physical_device);
        self.vulkan_device = Some(device);
        self.vulkan_queue_familes = Some(queue_family);

        Ok(())
    }
}

impl<'a> ContextWrapper<'a> for VulkanContext<'a> {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("Vulkan context loading symbols via ___");
        Ok(())
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        self
    }

    fn create_window_surface(&mut self, glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        self.create_window_surface(glfw_w)
    }

    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        self.create_devices()
    }
}
