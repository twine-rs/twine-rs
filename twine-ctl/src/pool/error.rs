// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::device::{DeviceCapability, DeviceId};
use thiserror::Error;

/// Errors that can occur when interacting with a [`DevicePool`](super::DevicePool).
#[derive(Debug, Error)]
pub enum DevicePoolError {
    /// A device with the given ID already exists in the pool.
    #[error("device `{0}` already exists in the pool")]
    DuplicateDevice(DeviceId),

    /// No device with the given ID was found in the pool.
    #[error("device `{0}` not found in the pool")]
    DeviceNotFound(DeviceId),

    /// The requested device is currently reserved by another handle.
    #[error("device `{0}` is already reserved")]
    DeviceUnavailable(DeviceId),

    /// The device is not reserved by the calling handle.
    #[error("device `{0}` is not reserved by this handle")]
    NotReservedByHandle(DeviceId),

    /// No unreserved devices remain in the pool.
    #[error("no devices available in the pool")]
    NoDevicesAvailable,

    /// No unreserved device with the requested capability is available.
    #[error("no available device with capability `{0:?}`")]
    NoDeviceWithCapability(DeviceCapability),
}
