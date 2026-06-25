// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

use twine_rs_macros::Tlv;

const PSKC_MAX_SIZE: usize = 16;

/// A Thread PSKc
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x04, tlv_length = 16, derive_inner)]
pub struct Pskc([u8; PSKC_MAX_SIZE]);

impl Pskc {
    pub fn random() -> Self {
        let mut bytes = [0u8; PSKC_MAX_SIZE];
        crate::fill_random_bytes(&mut bytes);
        Self(bytes)
    }
}

impl core::fmt::Display for Pskc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

#[cfg(any(test, feature = "alloc"))]
impl From<Pskc> for Vec<u8> {
    fn from(value: Pskc) -> Self {
        value.0.to_vec()
    }
}

impl From<Pskc> for u128 {
    fn from(value: Pskc) -> Self {
        u128::from_be_bytes(value.0)
    }
}

impl From<u128> for Pskc {
    fn from(pskc: u128) -> Self {
        Self(pskc.to_be_bytes())
    }
}

impl From<[u8; PSKC_MAX_SIZE]> for Pskc {
    fn from(value: [u8; PSKC_MAX_SIZE]) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn display_all_zeros() {
        assert_eq!(
            format!("{}", Pskc::from(0u128)),
            "00000000000000000000000000000000"
        );
    }

    #[test]
    fn display_known_value() {
        let pskc = Pskc::from([
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99,
        ]);
        assert_eq!(format!("{}", pskc), "aabbccddeeff00112233445566778899");
    }

    #[test]
    fn roundtrip_u128() {
        let value = 0xaabb_ccdd_eeff_0011_2233_4455_6677_8899_u128;
        assert_eq!(u128::from(Pskc::from(value)), value);
    }

    #[test]
    fn roundtrip_array() {
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ];
        assert_eq!(Vec::<u8>::from(Pskc::from(bytes)), bytes);
    }

    #[test]
    fn from_u128_be_byte_order() {
        let pskc = Pskc::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10_u128);
        let bytes: Vec<u8> = pskc.into();
        assert_eq!(bytes[0], 0x01);
        assert_eq!(bytes[15], 0x10);
    }
}
