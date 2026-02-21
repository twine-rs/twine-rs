// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serialport::{available_ports, SerialPortInfo, SerialPortType};

use crate::error::TwineCtlError;

pub struct SerialDeviceInfo {
    pub vendor: &'static str,
    pub product: Option<&'static str>,
    pub vid: u16,
    pub pid: u16,
}

pub const THREAD_REFERENCE_DEVICE_LIST: &[SerialDeviceInfo] = &[
    SerialDeviceInfo {
        vendor: "Nordic Semiconductor",
        product: Some("nRF528xx OpenThread Device"),
        vid: 0x1915,
        pid: 0xcafe,
    },
];

pub fn list_serial_devices(
    filter: &[SerialDeviceInfo],
) -> Result<Vec<SerialPortInfo>, TwineCtlError> {
    let matching_ports: Vec<_> = available_ports()?
        .into_iter()
        .filter(|p| match &p.port_type {
            SerialPortType::UsbPort(info) => {
                #[cfg(target_os = "macos")]
                if p.port_name.starts_with("/dev/tty") {
                    return false;
                }

                filter.iter().any(|device| {
                    device.vid == info.vid
                        && device.pid == info.pid
                        && device.product == info.product.as_deref()
                })
            }
            _ => false,
        })
        .collect();

    Ok(matching_ports)
}
