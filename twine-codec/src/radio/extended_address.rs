// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

const EXTENDED_ADDRESS_SIZE: usize = 8;

pub struct ExtendedAddress([u8; EXTENDED_ADDRESS_SIZE]);

impl ExtendedAddress {
    pub fn random() -> Self {
        let mut bytes = [0u8; EXTENDED_ADDRESS_SIZE];
        crate::fill_random_bytes(&mut bytes);
        Self(bytes)
    }
}

impl From<ExtendedAddress> for u64 {
    fn from(value: ExtendedAddress) -> Self {
        u64::from_be_bytes(value.0)
    }
}

impl From<u64> for ExtendedAddress {
    fn from(extended_address: u64) -> Self {
        Self(extended_address.to_be_bytes())
    }
}

impl core::fmt::Display for ExtendedAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}
