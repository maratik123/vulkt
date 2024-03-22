use crate::app_error::{AppError, QueueFamilyType};
use crate::vulkan::physical_device::DEVICE_EXTENSIONS;
use crate::vulkan::QueueFamilyIndices;
use anyhow::Result;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Features, Queue, QueueCreateInfo};

pub struct AppLogicalDevice {
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
    pub present_queue: Arc<Queue>,
}

impl AppLogicalDevice {
    pub fn create(
        physical_device: &Arc<PhysicalDevice>,
        queue_family_indices: &QueueFamilyIndices,
    ) -> Result<AppLogicalDevice> {
        let queue_create_infos = HashSet::from([
            queue_family_indices.graphics_family,
            queue_family_indices.present_family,
        ])
        .into_iter()
        .map(|queue_family_index| QueueCreateInfo {
            queue_family_index,
            queues: vec![1.0],
            ..QueueCreateInfo::default()
        })
        .collect();
        let device_features = Features::default();
        let device_create_info = DeviceCreateInfo {
            queue_create_infos,
            enabled_features: device_features,
            enabled_extensions: DEVICE_EXTENSIONS,
            ..DeviceCreateInfo::default()
        };
        let (device, queues) = Device::new(physical_device.clone(), device_create_info)?;
        let queues: SmallVec<[_; 2]> = queues.collect();

        fn find_queue<'a>(
            queues: impl IntoIterator<Item = &'a Arc<Queue>>,
            queue_family_index: u32,
            queue_type: QueueFamilyType,
        ) -> Result<Arc<Queue>> {
            Ok(queues
                .into_iter()
                .find(|queue| queue.queue_family_index() == queue_family_index)
                .ok_or(AppError::QueueForDevice(queue_type))?
                .clone())
        }

        Ok(AppLogicalDevice {
            device,
            graphics_queue: find_queue(
                &queues,
                queue_family_indices.graphics_family,
                QueueFamilyType::Graphics,
            )?,
            present_queue: find_queue(
                &queues,
                queue_family_indices.present_family,
                QueueFamilyType::Present,
            )?,
        })
    }
}
