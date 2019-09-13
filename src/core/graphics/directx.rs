use std::any::Any;
use std::sync::Arc;

use vulkano::swapchain::Surface;

#[cfg(windows)]
use dxplr;
#[cfg(windows)]
use dxplr::d3d::IBlob;
#[cfg(windows)]
use dxplr::d3d12::{ ICommandAllocator, CommandAllocator, ICommandQueue, CommandQueue, IDebug,
                    Debug, IDescriptorHeap, DescriptorHeap, GPUDescriptorHandle, CPUDescriptorHandle, IDevice, Device,
                    IFence, Fence, IGraphicsCommandList, GraphicsCommandList, IResource, Resource };
#[cfg(windows)]
use dxplr::{d3d, d3d12, dxgi, d3d12_input_layout_descs, EventHandle, Interface};
#[cfg(windows)]
use dxplr::dxgi::{ IFactory2, Factory6, ISwapChain, SwapChain4};

use crate::core::graphics::{ContextWrapper, SymbolLoadError, DeviceCreationError};

#[cfg(windows)]
pub struct DirectXContext {
    gl: glfw::Glfw,
    dx_debug: Debug,
    dx_device: Device,
    dx_command_alloc: CommandAllocator,
    dx_command_queue: CommandQueue,
    dx_graphics_command_list: GraphicsCommandList,
    dx_descriptor_heap: DescriptorHeap,
    dx_gpu_handle: GPUDescriptorHandle,
    dx_cpu_handle: CPUDescriptorHandle,
    dx_fence: Fence,
    dx_event_handle: EventHandle,
    dx_dxgi_factory: Factory6,
    dx_dxgi_swapchain: SwapChain4,
}

#[cfg(windows)]
impl<'a> DirectXContext {
    pub fn new( glfw_w: glfw::Window) -> (DirectXContext, glfw::Window) {
        let d3d12_debug = {
            let debug = d3d12::get_debug_interface::<d3d12::Debug>().unwrap();
            debug.enable_debug_layer();
            debug
        };
        let device = d3d12::create_device::<d3d12::Device>(None, d3d::FeatureLevel(12, 0)).unwrap();
        let command_alloc = device.create_command_allocator::<d3d12::CommandAllocator>(d3d12::CommandListType::Direct).unwrap();
        let command_queue = device.create_command_queue::<d3d12::CommandQueue>(&d3d12::CommandQueueDesc::new().
                            list_type(d3d12::CommandListType::Direct))
                            .unwrap();
        let command_list = device.create_command_list::<d3d12::GraphicsCommandList>(0, d3d12::CommandListType::Direct,
                            &command_alloc, None).unwrap();
        command_list.close();
        let dxgi_factory = dxgi::create_dxgi_factory2::<dxgi::Factory6>(None).unwrap();
        let mut window_size = glfw_w.get_framebuffer_size();
        let dpi_factor = glfw_w.get_content_scale();
        window_size = ((window_size.0 as f32 * dpi_factor.0) as i32, (window_size.1 as f32 * dpi_factor.1) as i32);
        let swapchain = dxgi_factory.create_swap_chain_for_hwnd(&command_queue, &glfw_w.get_win32_window(),
                            &dxgi::SwapChainDesc1::new()
                            .width(window_size.0 as u32)
                            .height(window_size.1 as u32)
                            .format(dxgi::Format::R8G8B8A8Unorm)
                            .buffer_usage(dxgi::Usage::RenderTargetOutput)
                            .buffer_count(2)
                            .swap_effect(dxgi::SwapEffect::FlipDiscard),
                            None, None)
                            .unwrap()
                            .query_interface::<dxgi::SwapChain4>()
                            .unwrap();
        let rtv_heap = device.create_descriptor_heap::<d3d12::DescriptorHeap>(&d3d12::DescriptorHeapDesc::new()
                                                        .heap_type(d3d12::DescriptorHeapType::RTV)
                                                        .num_descriptors(2)).unwrap();
        let gpu_handle = rtv_heap.get_gpu_descriptor_handle_for_heap_start();
        let cpu_handle = rtv_heap.get_cpu_descriptor_handle_for_heap_start();
        let fence = device.create_fence::<d3d12::Fence>(0, None).unwrap();
        let event = dxplr::EventHandle::new();


        (DirectXContext { gl: glfw_w.glfw, dx_debug: d3d12_debug, dx_device: device, dx_command_alloc: command_alloc,
                        dx_command_queue: command_queue, dx_graphics_command_list: command_list,
                        dx_descriptor_heap: rtv_heap, dx_gpu_handle: gpu_handle,
                        dx_cpu_handle: cpu_handle, dx_fence: fence, dx_event_handle: event,
                        dx_dxgi_factory: dxgi_factory, dx_dxgi_swapchain: swapchain }, glfw_w)
    }
}

#[cfg(unix)]
pub struct DirectXContext {

}

#[cfg(unix)]
impl DirectXContext {
    pub fn new(glfw_w: glfw::Window) -> (DirectXContext, glfw::Window) {
        (DirectXContext{}, glfw_w)
    }
}

impl<'a> ContextWrapper<'a> for DirectXContext {
    fn load_symbols(&mut self) -> Result<(), SymbolLoadError> {
        debug!("DirectX context loading symbols with __");
        Ok(())
    }

    fn as_any(&'a mut self) -> &'a mut (dyn Any +'a) {
        self
    }

    fn create_window_surface(&mut self, _glfw_w: glfw::Window) -> Option<Arc<Surface<glfw::Window>>> {
        None
    }

    fn create_devices(&'a mut self) -> Result<(), DeviceCreationError> {
        Err(DeviceCreationError::NotVulkanContext)
    }
}
