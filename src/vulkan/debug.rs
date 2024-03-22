use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};
use vulkano::instance::debug::{
    DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
    DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
};
use vulkano::instance::Instance;

#[inline]
pub fn populate_debug_utils_messenger_create_info() -> DebugUtilsMessengerCreateInfo {
    DebugUtilsMessengerCreateInfo {
        message_severity: DebugUtilsMessageSeverity::ERROR
            | DebugUtilsMessageSeverity::WARNING
            | DebugUtilsMessageSeverity::INFO
            | DebugUtilsMessageSeverity::VERBOSE,
        message_type: DebugUtilsMessageType::GENERAL
            | DebugUtilsMessageType::VALIDATION
            | DebugUtilsMessageType::PERFORMANCE,
        ..DebugUtilsMessengerCreateInfo::user_callback(debug_utils_messenger_callback())
    }
}

#[inline]
fn debug_utils_messenger_callback() -> Arc<DebugUtilsMessengerCallback> {
    // SAFETY: func does not make any calls to the Vulkan API
    unsafe {
        DebugUtilsMessengerCallback::new(|message_severity, message_type, callback_data| {
            if message_severity.intersects(DebugUtilsMessageSeverity::ERROR) {
                error!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::WARNING) {
                warn!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::INFO) {
                info!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::VERBOSE) {
                debug!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else {
                trace!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            }
        })
    }
}

#[inline]
pub fn setup_debug_messenger(instance: &Arc<Instance>) -> Result<DebugUtilsMessenger> {
    Ok(DebugUtilsMessenger::new(
        instance.clone(),
        populate_debug_utils_messenger_create_info(),
    )?)
}
