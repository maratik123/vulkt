use std::num::ParseIntError;
use thiserror::Error;
use vulkano::{LoadingError, Validated, VulkanError};

pub type AppResult<R> = Result<R, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("no local Vulkan library: {0:?}")]
    LibraryUnavailable(#[from] LoadingError),
    #[error("Vulkan or validation error: {0:?}")]
    ValidatedVulkan(#[from] Validated<VulkanError>),
    #[error("Vulkan error: {0:?}")]
    Vulkan(#[from] VulkanError),
    #[error("can not parse str: {0:?}")]
    ParseInt(#[from] ParseIntError),
    #[error("has not required layers")]
    RequiredLayers,
    #[error("failed to find a suitable physical device")]
    PhysicalDevices,
    #[error("failed to get a queue for logical device")]
    QueueForDevice,
}
