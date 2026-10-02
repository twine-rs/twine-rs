// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;
use twine_tlv::{
    DecodeTlvUnchecked as _, DecodeTlvValueUnchecked, TryEncodeTlv as _, TryEncodeTlvValue,
    TwineTlvError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x01, tlv_length = 1)]
struct Example([u8; 1]);

impl DecodeTlvValueUnchecked for Example {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        Example(<[u8; 1]>::decode_tlv_value_unchecked(buffer))
    }
}

impl TryEncodeTlvValue for Example {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0.try_encode_tlv_value(buffer)
    }
}

fn main() {
    let value = Example([0xAA]);
    let mut buffer = [0u8; 3];

    let written = value.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, 3);
    assert_eq!(buffer, [0x01, 0x01, 0xAA]);

    assert_eq!(Example::decode_tlv_unchecked(buffer), value);
}
