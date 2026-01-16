// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_macros::Tlv;
use twine_tlv::prelude::*;

pub(crate) const EXT_PAN_ID_SIZE: usize = 8;

/// IEEE 802.15.4 Extended PAN ID
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x02, tlv_length = 8, derive_inner)]
pub struct ExtendedPanId([u8; EXT_PAN_ID_SIZE]);

impl ExtendedPanId {
    pub fn random() -> Self {
        let mut bytes = [0u8; EXT_PAN_ID_SIZE];
        crate::fill_random_bytes(&mut bytes);
        Self(bytes)
    }
}

impl From<ExtendedPanId> for u64 {
    fn from(value: ExtendedPanId) -> Self {
        u64::from_be_bytes(value.0)
    }
}

impl From<u64> for ExtendedPanId {
    fn from(id: u64) -> Self {
        Self(id.to_be_bytes())
    }
}

impl From<ExtendedPanId> for [u8; EXT_PAN_ID_SIZE] {
    fn from(value: ExtendedPanId) -> Self {
        value.0
    }
}

impl From<[u8; EXT_PAN_ID_SIZE]> for ExtendedPanId {
    fn from(value: [u8; EXT_PAN_ID_SIZE]) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for ExtendedPanId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use twine_tlv::prelude::*;

    use super::*;

    const TEST_VALUE: u64 = 0x1122334455667788;

    #[test]
    fn xpan_transform_u64() {
        let test = TEST_VALUE;
        let xpan = ExtendedPanId::from(test);
        let result = u64::from(xpan);
        assert_eq!(test, result);
    }

    #[test]
    fn xpan_transform_array() {
        let test = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        let xpan = ExtendedPanId::from(test);
        let result: [u8; EXT_PAN_ID_SIZE] = xpan.into();
        assert_eq!(test, result);
        assert_eq!(TEST_VALUE, xpan.into());
    }

    #[test]
    fn xpan_encode_decode_tlv() {
        let test = TEST_VALUE;
        let xpan = ExtendedPanId::from(test);
        let mut collection = TlvCollection::<16>::default();
        collection.push(xpan).unwrap();
        insta::assert_debug_snapshot!(format!("{:02x?}", collection));
        let result = collection.decode_type_unchecked::<ExtendedPanId>().unwrap();
        assert_eq!(xpan, result);
    }
}
