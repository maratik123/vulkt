mod debug;
mod instance;
mod logical_device;
mod physical_device;
mod queue_family_indices;
mod surface;
mod swapchain;

use crate::app_result::AppResult;
use crate::vulkan::debug::setup_debug_messenger;
use crate::vulkan::instance::create_instance;
use crate::vulkan::logical_device::AppLogicalDevice;
use crate::vulkan::physical_device::pick_physical_device;
use crate::vulkan::queue_family_indices::QueueFamilyIndices;
use crate::vulkan::surface::create_surface;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, Queue};
use vulkano::image::Image;
use vulkano::instance::debug::DebugUtilsMessenger;
use vulkano::instance::Instance;
use vulkano::swapchain::{Surface, Swapchain};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct AppVulkan {
    pub swapchain_images: Vec<Arc<Image>>,
    pub swapchain: Arc<Swapchain>,
    pub present_queue: Arc<Queue>,
    pub graphics_queue: Arc<Queue>,
    pub device: Arc<Device>,
    pub physical_device: Arc<PhysicalDevice>,
    pub surface: Arc<Surface>,
    pub debug_utils_messenger: Option<DebugUtilsMessenger>,
    pub instance: Arc<Instance>,
}

impl AppVulkan {
    pub fn init(
        event_loop: &EventLoop<()>,
        window: Arc<Window>,
        enable_validation: bool,
    ) -> AppResult<Self> {
        let instance = create_instance(event_loop, enable_validation)?;
        let debug_utils_messenger = if enable_validation {
            Some(setup_debug_messenger(instance.clone())?)
        } else {
            None
        };
        let surface = create_surface(instance.clone(), window.clone())?;
        let (physical_device, queue_family_indices, swap_chain_support) =
            pick_physical_device(&instance, &surface)?;
        let AppLogicalDevice {
            device,
            graphics_queue,
            present_queue,
            ..
        } = AppLogicalDevice::create(physical_device.clone(), &queue_family_indices)?;
        let (swapchain, swapchain_images) = swap_chain_support.create_swapchain(
            device.clone(),
            surface.clone(),
            &window,
            &queue_family_indices,
        )?;

        Ok(Self {
            instance,
            debug_utils_messenger,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            swapchain,
            swapchain_images,
        })
    }
}
