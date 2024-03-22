use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum QueueFamilyType {
    Graphics,
    Present,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("has not required layers")]
    RequiredLayers,
    #[error("failed to find a suitable physical device")]
    PhysicalDevices,
    #[error("failed to get a {0:?} queue for logical device")]
    QueueForDevice(QueueFamilyType),
    #[error("no available swap chain formats")]
    SwapChainFormatUnavailable,
    #[error("no entry point found")]
    EntryPointNotFound,
}
