// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Capabilities of a device.
///
/// Used when registering a device and when requesting a device by capability
/// rather than by a specific
/// [`DeviceId`](crate::devpool::DeviceId).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeviceCapability {
    /// A full-function Thread device (router-capable).
    FullFunctionDevice,

    /// A backbone border router.
    BackboneBorderRouter,

    /// A reduced-function (sleepy end) device.
    ReducedFunctionDevice,
}
