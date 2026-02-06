// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use bitflags::bitflags;
use bytes::{Buf, BufMut};
use twine_macros::Tlv;
use twine_tlv::prelude::*;
use twine_tlv::{DecodeTlvValueUnchecked, TryEncodeTlv, TryEncodeTlvValue};
use typed_builder::TypedBuilder;

use crate::TwineCodecError;

bitflags! {
    /// IEEE 802.15.4 channel page mask
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct ChannelPageMask: u8 {
        /// 2.4 GHz O-QPSK PHY
        const PAGE_0 = 0;
    }

    /// IEEE 802.15.4 Channel Mask
    ///
    /// Each bit in the channel mask represents the selected channel.
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub struct ChannelMaskBits: u32 {
        const CHANNEL_0 = 1 << 0;
        const CHANNEL_1 = 1 << 1;
        const CHANNEL_2 = 1 << 2;
        const CHANNEL_3 = 1 << 3;
        const CHANNEL_4 = 1 << 4;
        const CHANNEL_5 = 1 << 5;
        const CHANNEL_6 = 1 << 6;
        const CHANNEL_7 = 1 << 7;
        const CHANNEL_8 = 1 << 8;
        const CHANNEL_9 = 1 << 9;
        const CHANNEL_10 = 1 << 10;
        const CHANNEL_11 = 1 << 11;
        const CHANNEL_12 = 1 << 12;
        const CHANNEL_13 = 1 << 13;
        const CHANNEL_14 = 1 << 14;
        const CHANNEL_15 = 1 << 15;
        const CHANNEL_16 = 1 << 16;
        const CHANNEL_17 = 1 << 17;
        const CHANNEL_18 = 1 << 18;
        const CHANNEL_19 = 1 << 19;
        const CHANNEL_20 = 1 << 20;
        const CHANNEL_21 = 1 << 21;
        const CHANNEL_22 = 1 << 22;
        const CHANNEL_23 = 1 << 23;
        const CHANNEL_24 = 1 << 24;
        const CHANNEL_25 = 1 << 25;
        const CHANNEL_26 = 1 << 26;
        const CHANNEL_27 = 1 << 27;
        const CHANNEL_28 = 1 << 28;
        const CHANNEL_29 = 1 << 29;
        const CHANNEL_30 = 1 << 30;
        const CHANNEL_31 = 1 << 31;
    }
}

impl From<ChannelPageMask> for u8 {
    fn from(value: ChannelPageMask) -> Self {
        value.bits()
    }
}

impl From<u8> for ChannelPageMask {
    fn from(value: u8) -> Self {
        match value {
            0 => ChannelPageMask::PAGE_0,
            _ => panic!("Invalid channel page mask: {}", value),
        }
    }
}

/// IEEE 802.15.4 Channel Mask TLV
///
/// FIXME: Currently only supports a single channel mask entry.
///
/// The TLV is required to support multiple channel mask entries, but
/// practically, this is rarely used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Tlv, TypedBuilder)]
#[tlv(tlv_type = 0x35, tlv_length = 6)]
pub struct ChannelMask {
    page: ChannelPageMask,
    len: u8,
    mask: ChannelMaskBits,
}

impl Default for ChannelMask {
    fn default() -> Self {
        let mask = ChannelMaskBits::CHANNEL_11
            | ChannelMaskBits::CHANNEL_12
            | ChannelMaskBits::CHANNEL_13
            | ChannelMaskBits::CHANNEL_14
            | ChannelMaskBits::CHANNEL_15
            | ChannelMaskBits::CHANNEL_16
            | ChannelMaskBits::CHANNEL_17
            | ChannelMaskBits::CHANNEL_18
            | ChannelMaskBits::CHANNEL_19
            | ChannelMaskBits::CHANNEL_20
            | ChannelMaskBits::CHANNEL_21
            | ChannelMaskBits::CHANNEL_22
            | ChannelMaskBits::CHANNEL_23
            | ChannelMaskBits::CHANNEL_24
            | ChannelMaskBits::CHANNEL_25
            | ChannelMaskBits::CHANNEL_26;

        ChannelMask {
            page: ChannelPageMask::PAGE_0,
            len: 4,
            mask,
        }
    }
}

impl ChannelMask {
    pub fn mask(&self) -> u32 {
        self.mask.bits()
    }
}

impl core::fmt::Display for ChannelMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08x}", self.mask())
    }
}

impl FromStr for ChannelMask {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);
        let mask = u32::from_str_radix(s, 16).map_err(|_| TwineCodecError::StringParseError)?;
        Ok(ChannelMask {
            page: ChannelPageMask::PAGE_0,
            len: 4,
            mask: ChannelMaskBits::from_bits_truncate(mask),
        })
    }
}

impl DecodeTlvValueUnchecked for ChannelMask {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let page = buffer.get_u8();
        let mask_len = buffer.get_u8();
        let mask_bytes = buffer.get_u32();

        // Reverse the bits of the channel mask to match the TLV format listed in the Thread 1.4.0 specification.
        //
        // Thread 1.4.0 8.10.1.18.1 Channel Mask Entry
        let mask = ChannelMaskBits::from_bits_truncate(mask_bytes.reverse_bits());

        Self {
            page: page.into(),
            len: mask_len,
            mask,
        }
    }
}

impl TryEncodeTlvValue for ChannelMask {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, twine_tlv::TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u8(self.page.into());
        buffer.put_u8(self.len);
        buffer.put_u32(self.mask.bits().reverse_bits());
        Ok(self.tlv_len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_default() {
        let default = ChannelMask::default();
        assert_eq!(default.page, ChannelPageMask::PAGE_0);
        assert_eq!(default.len, 4);
        assert_eq!(default.mask(), 0x07FF_F800);
    }

    #[test]
    fn success_decode_tlv() {
        let tlv_bytes: [u8; 8] = [53, 6, 0, 4, 0, 31, 255, 224];
        let channel_mask = ChannelMask::decode_tlv_unchecked(&tlv_bytes);
        assert_eq!(channel_mask.page, ChannelPageMask::PAGE_0);
        assert_eq!(channel_mask.len, 4);
        assert_eq!(channel_mask.mask(), 0x07FF_F800);
    }

    #[test]
    fn success_encode_tlv() {
        let channel_mask = ChannelMask::default();
        let mut buffer = [0_u8; 8];
        let bytes_written = channel_mask
            .try_encode_tlv(&mut buffer)
            .expect("Could not encode ChannelMask");
        assert_eq!(bytes_written, channel_mask.tlv_total_len());
        let expected_bytes: [u8; 8] = [53, 6, 0, 4, 0, 31, 255, 224];
        assert_eq!(expected_bytes.as_ref(), &buffer[..bytes_written]);
    }
}
