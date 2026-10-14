//! Core domain types and USB packet definitions for Redragon M709.

pub mod constants;
pub mod packets;
pub mod types;

pub use constants::*;
pub use packets::*;
pub use types::*;

pub const VENDOR_ID: u16 = 0x04D9;
pub const PRODUCT_ID: u16 = 0xFC2A;
pub const INTERFACE_INDEX: u16 = 2;
pub const MODEL_NAME: &str = "Redragon M709";
