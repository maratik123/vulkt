use crate::app_result::{AppError, AppResult};
use crate::vulkan::queue_family_indices::QueueFamilyIndices;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageUsage};
use vulkano::swapchain::{
    ColorSpace, CompositeAlpha, PresentMode, Surface, SurfaceCapabilities, SurfaceInfo, Swapchain,
    SwapchainCreateInfo,
};
use vulkano::sync::Sharing;
use winit::window::Window;

pub struct SwapChainSupportDetails {
    capabilities: SurfaceCapabilities,
    formats: Vec<(Format, ColorSpace)>,
    present_modes: Vec<PresentMode>,
}

impl SwapChainSupportDetails {
    pub fn query(physical_device: &PhysicalDevice, surface: &Surface) -> AppResult<Self> {
        let capabilities = physical_device.surface_capabilities(surface, SurfaceInfo::default())?;
        let formats = physical_device
            .surface_formats(surface, SurfaceInfo::default())?
            .into_iter()
            .collect();
        let present_modes = physical_device
            .surface_present_modes(surface, SurfaceInfo::default())?
            .collect();
        Ok(SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    #[inline]
    pub fn is_adequate(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }

    pub fn create_swapchain(
        self,
        device: &Arc<Device>,
        surface: &Arc<Surface>,
        window: &Window,
        queue_family_indices: &QueueFamilyIndices,
    ) -> AppResult<(Arc<Swapchain>, Vec<Arc<Image>>)> {
        let (image_format, image_color_space) = choose_swap_surface_format(self.formats)?;
        let present_mode = choose_swap_present_mode(self.present_modes);
        let image_extent = choose_swap_extent(&self.capabilities, window);
        let mut min_image_count = self.capabilities.min_image_count + 1;
        if let Some(max_image_count) = self.capabilities.max_image_count {
            min_image_count = min_image_count.min(max_image_count);
        }
        let image_sharing =
            if queue_family_indices.graphics_family == queue_family_indices.present_family {
                Sharing::Exclusive
            } else {
                Sharing::Concurrent(
                    [
                        queue_family_indices.graphics_family,
                        queue_family_indices.present_family,
                    ]
                    .into_iter()
                    .collect(),
                )
            };
        let swapchain_create_info = SwapchainCreateInfo {
            min_image_count,
            image_format,
            image_color_space,
            image_extent,
            image_array_layers: 1,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            image_sharing,
            pre_transform: self.capabilities.current_transform,
            composite_alpha: CompositeAlpha::Opaque,
            present_mode,
            clipped: true,
            ..SwapchainCreateInfo::default()
        };
        Ok(Swapchain::new(
            device.clone(),
            surface.clone(),
            swapchain_create_info,
        )?)
    }
}

fn choose_swap_surface_format(
    available_formats: impl IntoIterator<Item = (Format, ColorSpace)>,
) -> AppResult<(Format, ColorSpace)> {
    let mut iter = available_formats.into_iter();
    let first = iter.next();
    first
        .iter()
        .copied()
        .chain(iter)
        .find(|format| format == &(Format::B8G8R8A8_SRGB, ColorSpace::SrgbNonLinear))
        .or(first)
        .ok_or(AppError::SwapChainFormatUnavailable)
}

fn choose_swap_present_mode(
    available_present_modes: impl IntoIterator<Item = PresentMode>,
) -> PresentMode {
    available_present_modes
        .into_iter()
        .find(|&mode| mode == PresentMode::Mailbox)
        .unwrap_or(PresentMode::Fifo)
}

fn choose_swap_extent(surface_capabilities: &SurfaceCapabilities, window: &Window) -> [u32; 2] {
    if let Some(current_extent) = surface_capabilities.current_extent {
        current_extent
    } else {
        let [x, y]: [u32; 2] = window.inner_size().into();
        let [min_x, min_y] = surface_capabilities.min_image_extent;
        let [max_x, max_y] = surface_capabilities.max_image_extent;
        [x.clamp(min_x, max_x), y.clamp(min_y, max_y)]
    }
}

pub fn create_image_views(swapchain_images: &[Arc<Image>]) -> AppResult<Vec<Arc<ImageView>>> {
    Ok(swapchain_images
        .iter()
        .map(|image| ImageView::new_default(image.clone()))
        .collect::<Result<_, _>>()?)
}
