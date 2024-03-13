use crate::app_result::AppResult;
use vulkano::device::physical::PhysicalDevice;
use vulkano::format::Format;
use vulkano::swapchain::{PresentMode, Surface, SurfaceCapabilities, SurfaceInfo};

pub struct SwapChainSupportDetails {
    _capabilities: SurfaceCapabilities,
    formats: Vec<Format>,
    present_mode: Vec<PresentMode>,
}

impl SwapChainSupportDetails {
    pub fn query(physical_device: &PhysicalDevice, surface: &Surface) -> AppResult<Self> {
        let capabilities = physical_device.surface_capabilities(surface, SurfaceInfo::default())?;
        let formats = physical_device
            .surface_formats(surface, SurfaceInfo::default())?
            .into_iter()
            .map(|(format, _)| format)
            .collect();
        let present_mode = physical_device
            .surface_present_modes(surface, SurfaceInfo::default())?
            .collect();
        Ok(SwapChainSupportDetails {
            _capabilities: capabilities,
            formats,
            present_mode,
        })
    }

    pub fn is_adequate(&self) -> bool {
        !self.formats.is_empty() && !self.present_mode.is_empty()
    }
}
