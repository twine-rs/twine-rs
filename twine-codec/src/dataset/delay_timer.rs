// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;

/// An unsigned 32-bit number representing the time delay before the pending
/// dataset to be applied, in milliseconds.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x34, tlv_length = 4, derive_inner)]
pub struct DelayTimer(u32);

impl DelayTimer {
    pub fn duration(&self) -> core::time::Duration {
        core::time::Duration::from_millis(self.0 as u64)
    }
}

impl From<u32> for DelayTimer {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    #[test]
    fn duration_zero() {
        assert_eq!(DelayTimer::from(0).duration(), Duration::from_millis(0));
    }

    #[test]
    fn duration_typical() {
        assert_eq!(
            DelayTimer::from(30_000).duration(),
            Duration::from_millis(30_000)
        );
    }

    #[test]
    fn duration_max() {
        assert_eq!(
            DelayTimer::from(u32::MAX).duration(),
            Duration::from_millis(u32::MAX as u64)
        );
    }

    #[test]
    fn roundtrip_from_u32() {
        let millis = 12_345_u32;
        assert_eq!(DelayTimer::from(millis), DelayTimer::from(millis));
    }
}
