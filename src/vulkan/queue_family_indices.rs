use crate::app_result::AppResult;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::QueueFlags;
use vulkano::swapchain::Surface;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct QueueFamilyIndices {
    pub graphics_family: u32,
    pub present_family: u32,
}

impl QueueFamilyIndices {
    pub fn find(
        physical_device: &PhysicalDevice,
        surface: &Surface,
    ) -> AppResult<Option<QueueFamilyIndices>> {
        let mut queue_family_indices = QueueFamilyIndicesBuilder::default();
        for (i, prop) in physical_device.queue_family_properties().iter().enumerate() {
            let i = i as u32;
            let mut changed = false;

            if queue_family_indices.graphics_family.is_none()
                && prop.queue_flags.contains(QueueFlags::GRAPHICS)
            {
                changed |= true;
                queue_family_indices.graphics_family = Some(i);
            }

            if queue_family_indices.present_family.is_none()
                && physical_device.surface_support(i, surface)?
            {
                changed |= true;
                queue_family_indices.present_family = Some(i);
            }

            if changed {
                if let queue_family_indices @ Some(_) = queue_family_indices.build() {
                    return Ok(queue_family_indices);
                }
            }
        }
        Ok(None)
    }
}

#[derive(Default)]
struct QueueFamilyIndicesBuilder {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndicesBuilder {
    #[inline]
    fn build(&self) -> Option<QueueFamilyIndices> {
        Some(QueueFamilyIndices {
            graphics_family: self.graphics_family?,
            present_family: self.present_family?,
        })
    }
}
