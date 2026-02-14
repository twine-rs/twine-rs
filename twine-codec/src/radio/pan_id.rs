// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use twine_macros::Tlv;

use crate::TwineCodecError;

/// IEEE 802.15.4 PAN ID
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x01, tlv_length = 2, derive_inner)]
pub struct PanId(u16);

impl PanId {
    /// Create a new IEEE 802.15.4 PAN ID
    pub fn new(pan_id: u16) -> Self {
        Self(pan_id)
    }

    /// Create a new IEEE 802.15.4 Broadcast PAN ID
    pub fn broadcast() -> Self {
        Self(0xffff)
    }

    pub fn random() -> Self {
        let pan_id = crate::random_range_u16(0x0001..=0xfffe);
        Self(pan_id)
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

impl From<PanId> for u16 {
    fn from(value: PanId) -> Self {
        value.0
    }
}

impl From<u16> for PanId {
    fn from(pan_id: u16) -> Self {
        Self(pan_id)
    }
}

impl FromStr for PanId {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);
        let pan_id = u16::from_str_radix(s, 16).map_err(|_| TwineCodecError::StringParseError)?;
        Ok(Self::from(pan_id))
    }
}

impl core::fmt::Display for PanId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use twine_tlv::prelude::*;

    use super::*;

    const PAN_ID_TLV_BYTES: [u8; 4] = [0x01, 0x02, 0xde, 0xad];

    #[test]
    fn broadcast() {
        assert_eq!(PanId::broadcast(), PanId(0xffff));
    }

    #[test]
    fn success_try_decode_meshcop_tlv_for_pan_id() {
        let test = PanId::decode_tlv_unchecked(&PAN_ID_TLV_BYTES);
        assert_eq!(test.0, 0xdead);
    }

    #[test]
    fn success_try_encode_meshcop_tlv_for_pan_id() {
        let pan_id = PanId::new(0xdead);
        let mut test_buffer = [0_u8; 10];
        let bytes_written = pan_id
            .try_encode_tlv(&mut test_buffer)
            .expect("Could not encode PanId");
        assert_eq!(bytes_written, PanId::tlv_total_constant_len());
        assert_eq!(PAN_ID_TLV_BYTES.as_ref(), &test_buffer[..4]);
    }
}
