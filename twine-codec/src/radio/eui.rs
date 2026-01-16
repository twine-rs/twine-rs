// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

const EUI64_SIZE: usize = 8;

pub struct Eui64([u8; EUI64_SIZE]);

impl From<Eui64> for u64 {
    fn from(value: Eui64) -> Self {
        u64::from_be_bytes(value.0)
    }
}

impl From<u64> for Eui64 {
    fn from(extended_address: u64) -> Self {
        Self(extended_address.to_be_bytes())
    }
}
