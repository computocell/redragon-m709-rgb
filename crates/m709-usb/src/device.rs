use std::time::Duration;
use m709_core::{
    build_profile_packets, build_settings_packets, parse_settings,
    MouseConfig, Profile, DATA_READ_1, DATA_READ_2, DATA_READ_3,
    INTERFACE_INDEX, PRODUCT_ID, VENDOR_ID,
};
use rusb::{DeviceHandle, GlobalContext};
use crate::UsbError;

pub struct M709Device {
    handle: DeviceHandle<GlobalContext>,
}

impl M709Device {
    /// Opens the Redragon M709 device, detaches kernel driver from Interface 2 if necessary,
    /// and claims the interface for control transfers.
    pub fn open() -> Result<Self, UsbError> {
        let mut target_device = None;

        for device in rusb::devices()?.iter() {
            let desc = device.device_descriptor()?;
            if desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID {
                target_device = Some(device);
                break;
            }
        }

        let device = target_device.ok_or(UsbError::DeviceNotFound(VENDOR_ID, PRODUCT_ID))?;
        let handle = device.open()?;

        // Detach kernel driver from interface 2 if active
        let iface = INTERFACE_INDEX as u8;
        if handle.kernel_driver_active(iface).unwrap_or(false) {
            let _ = handle.detach_kernel_driver(iface);
        }

        handle.claim_interface(iface)?;

        Ok(Self { handle })
    }

    /// Changes active hardware profile (1 to 5).
    pub fn write_profile(&self, profile: Profile) -> Result<(), UsbError> {
        let packets = build_profile_packets(profile);
        let timeout = Duration::from_millis(1000);

        for packet in &packets {
            self.handle
                .write_control(0x21, 0x09, 0x0302, 0x0002, packet, timeout)?;
        }

        Ok(())
    }

    /// Writes entire mouse configuration (all profiles, lighting, DPI, buttons) to hardware.
    pub fn write_settings(&self, config: &MouseConfig) -> Result<(), UsbError> {
        let (b1, b2, b3) = build_settings_packets(config);
        let timeout = Duration::from_millis(1000);

        // Part 1: Lighting & report rates (15 packets of 16B)
        for packet in &b1 {
            self.handle
                .write_control(0x21, 0x09, 0x0302, 0x0002, packet, timeout)?;
        }

        // Part 2: Scroll speed (1 packet of 64B, report ID 0x03)
        self.handle
            .write_control(0x21, 0x09, 0x0303, 0x0002, &b2, timeout)?;

        // Part 3: DPI and button keymaps (80 packets of 16B)
        for packet in &b3 {
            self.handle
                .write_control(0x21, 0x09, 0x0302, 0x0002, packet, timeout)?;
        }

        Ok(())
    }

    /// Reads current configuration directly from mouse hardware EEPROM.
    pub fn read_settings(&self) -> Result<MouseConfig, UsbError> {
        let timeout = Duration::from_millis(1000);

        // Part 1
        let mut in1 = [[0u8; 16]; 8];
        self.handle
            .write_control(0x21, 0x09, 0x0302, 0x0002, &DATA_READ_1[0], timeout)?;

        for i in 1..9 {
            self.handle
                .write_control(0x21, 0x09, 0x0302, 0x0002, &DATA_READ_1[i], timeout)?;
            self.handle
                .read_control(0xA1, 0x01, 0x0302, 0x0002, &mut in1[i - 1], timeout)?;
        }

        // Part 2 (DPI and macro headers)
        let mut in2 = [[0u8; 64]; 85];
        for i in 0..85 {
            self.handle
                .write_control(0x21, 0x09, 0x0303, 0x0002, &DATA_READ_2[i], timeout)?;
            self.handle
                .read_control(0xA1, 0x01, 0x0303, 0x0002, &mut in2[i], timeout)?;
        }

        // Part 3 (Button keymaps)
        let mut in3 = [[0u8; 16]; 100];
        for i in 0..100 {
            self.handle
                .write_control(0x21, 0x09, 0x0302, 0x0002, &DATA_READ_3[i], timeout)?;
            self.handle
                .read_control(0xA1, 0x01, 0x0302, 0x0002, &mut in3[i], timeout)?;
        }

        // Final lock packet
        self.handle
            .write_control(0x21, 0x09, 0x0302, 0x0002, &DATA_READ_3[100], timeout)?;

        Ok(parse_settings(&in1, &in2, &in3))
    }
}

impl Drop for M709Device {
    fn drop(&mut self) {
        let _ = self.handle.release_interface(INTERFACE_INDEX as u8);
    }
}
