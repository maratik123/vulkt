use std::num::ParseIntError;
use thiserror::Error;
use vulkano::{LoadingError, Validated, VulkanError};
use winit::error::OsError;

pub type AppResult<R> = Result<R, AppError>;

#[derive(Debug, Clone, Copy)]
pub enum QueueFamilyType {
    Graphics,
    Present,
}

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
    #[error("failed to get a {0:?} queue for logical device")]
    QueueForDevice(QueueFamilyType),
    #[error("os error found: {0:?}")]
    OsError(#[from] OsError),
    #[error("no available swap chain formats")]
    SwapChainFormatUnavailable,
}
