// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::{Buf, BufMut};
use typed_builder::TypedBuilder;

use twine_macros::Tlv;
use twine_tlv::prelude::*;

use crate::error::TwineCodecError;

/// IEEE 802.15.4 channel
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Tlv, TypedBuilder)]
#[tlv(tlv_type = 0x00, tlv_length = 3)]
pub struct Channel {
    channel: u16,
    page: u8,
}

impl Channel {
    /// Generate a random page 0 channel between 11 and 26
    pub fn random() -> Self {
        let channel = crate::random_range_u16(11..=26);
        let page: u8 = 0;
        Self { channel, page }
    }

    pub fn new(page: u8, channel: u16) -> Self {
        Self { channel, page }
    }

    pub fn channel(&self) -> u16 {
        self.channel
    }

    pub fn page(&self) -> u8 {
        self.page
    }

    pub fn from_str_channel_only(s: &str) -> Result<Self, TwineCodecError> {
        let channel = u16::from_str_radix(s, 10).map_err(|_| TwineCodecError::StringParseError)?;
        Ok(Self::new(0, channel))
    }
}

impl DecodeTlvValueUnchecked for Channel {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let page = buffer.get_u8();
        let channel = buffer.get_u16();

        Self { channel, page }
    }
}

impl TryEncodeTlvValue for Channel {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u8(self.page());
        buffer.put_u16(self.channel());
        Ok(self.tlv_len())
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    const CHANNEL_TLV_BYTES: [u8; 5] = [0x00, 0x03, 0x00, 0x00, 0x16];

    #[test]
    fn test_channel_random() {
        let channel = Channel::random();
        assert!(channel.channel() >= 11 && channel.channel() <= 26);
        assert_eq!(channel.page(), 0);
    }

    #[test]
    fn success_decode_unchecked_meshcop_tlv_for_channel() {
        let test = Channel::decode_tlv_unchecked(&CHANNEL_TLV_BYTES);
        assert_eq!(test.page(), 0);
        assert_eq!(test.channel(), 22);
    }

    #[test]
    fn success_try_encode_meshcop_tlv_for_channel() {
        let channel = Channel::new(0, 22);
        let mut test_buffer = [0_u8; 10];
        let bytes_written = channel
            .try_encode_tlv(&mut test_buffer)
            .expect("Could not encode Channel");
        println!("Encoded Channel TLV: {:?}", &test_buffer[..]);
        assert_eq!(bytes_written, Channel::tlv_total_constant_len());
        assert_eq!(CHANNEL_TLV_BYTES.as_ref(), &test_buffer[..5]);
    }

    #[test]
    fn success_from_str_channel_only() {
        let channel =
            Channel::from_str_channel_only("16").expect("Could not parse Channel from string");
        assert_eq!(channel.page(), 0);
        assert_eq!(channel.channel(), 16);
    }
}
