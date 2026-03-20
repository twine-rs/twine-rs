// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use twine_rs_macros::Tlv;

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
    fn new_and_get() {
        assert_eq!(PanId::new(0x1234).get(), 0x1234);
    }

    #[test]
    fn roundtrip_u16() {
        let value = 0xabcd_u16;
        assert_eq!(u16::from(PanId::from(value)), value);
    }

    #[test]
    fn display() {
        use alloc::format;
        assert_eq!(format!("{}", PanId::new(0xdead)), "0xdead");
        assert_eq!(format!("{}", PanId::new(0x0001)), "0x0001");
    }

    #[test]
    fn from_str_without_prefix() {
        assert_eq!(PanId::from_str("dead").unwrap().get(), 0xdead);
    }

    #[test]
    fn from_str_with_prefix() {
        assert_eq!(PanId::from_str("0xdead").unwrap().get(), 0xdead);
    }

    #[test]
    fn from_str_invalid() {
        assert!(PanId::from_str("not_hex").is_err());
    }

    #[test]
    fn random_in_valid_range() {
        for _ in 0..100 {
            let pan_id = PanId::random();
            assert!(pan_id.get() >= 0x0001 && pan_id.get() <= 0xfffe);
        }
    }

    #[test]
    fn success_try_decode_meshcop_tlv_for_pan_id() {
        let test = PanId::decode_tlv_unchecked(PAN_ID_TLV_BYTES);
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
