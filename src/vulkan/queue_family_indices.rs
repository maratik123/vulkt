use crate::app_result::AppResult;
use tracing::info;
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
        let mut last_err = None;
        for (i, prop) in physical_device.queue_family_properties().iter().enumerate() {
            let i = i as u32;
            let mut changed = false;

            if queue_family_indices.graphics_family.is_none()
                && prop.queue_flags.contains(QueueFlags::GRAPHICS)
            {
                changed |= true;
                queue_family_indices.graphics_family = Some(i);
            }

            if queue_family_indices.present_family.is_none() {
                match physical_device.surface_support(i, surface) {
                    Ok(true) => {
                        changed |= true;
                        queue_family_indices.present_family = Some(i);
                    }
                    Ok(false) => (),
                    Err(e) => {
                        info!(
                            "matching physical device {physical_device:?} \
                             with surface {surface:?} at queue family index {i} \
                             ends with error {e}"
                        );
                        last_err = Some(e);
                    }
                }
            }

            if changed {
                if let queue_family_indices @ Some(_) = queue_family_indices.build() {
                    return Ok(queue_family_indices);
                }
            }
        }

        if let Some(err) = last_err {
            Err(err.into())
        } else {
            Ok(None)
        }
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
