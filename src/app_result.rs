use std::num::ParseIntError;
use thiserror::Error;
use vulkano::{LoadingError, Validated, VulkanError};

pub type AppResult<R> = Result<R, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("no local Vulkan library: {0:?}")]
    LibraryUnavailable(#[from] LoadingError),
    #[error("Vulkan error: {0:?}")]
    Vulkan(#[from] Validated<VulkanError>),
    #[error("can not parse str: {0:?}")]
    ParseInt(#[from] ParseIntError),
}
