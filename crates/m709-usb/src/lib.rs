//! USB driver implementation for Redragon M709.

use m709_core::{PRODUCT_ID, VENDOR_ID};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsbError {
    #[error("USB error: {0}")]
    Rusb(#[from] rusb::Error),
    #[error("Device not found (VID {0:04x}, PID {1:04x})")]
    DeviceNotFound(u16, u16),
}

/// Checks if a Redragon M709 is connected via USB.
pub fn is_device_connected() -> Result<bool, UsbError> {
    for device in rusb::devices()?.iter() {
        let desc = device.device_descriptor()?;
        if desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID {
            return Ok(true);
        }
    }
    Ok(false)
}
