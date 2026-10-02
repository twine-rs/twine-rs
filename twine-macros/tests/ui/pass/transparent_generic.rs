// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::{Tlv, TlvTransparent};
use twine_tlv::{
    DecodeTlvUnchecked as _, DecodeTlvValueUnchecked, TryEncodeTlv as _, TryEncodeTlvValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, TlvTransparent)]
struct Wrapper<T>(T);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv, TlvTransparent)]
#[tlv(tlv_type = 0x0A, tlv_length = 4, variants = [("Alt", tlv_type = 0x0B)])]
struct Paired<T>(T);

fn main() {
    let value = Wrapper(0xDEAD_BEEF_u32);
    let mut buffer = [0u8; 4];

    let written = value.try_encode_tlv_value(&mut buffer).expect("encode");
    assert_eq!(written, 4);
    assert_eq!(buffer, [0xDE, 0xAD, 0xBE, 0xEF]);

    assert_eq!(Wrapper::<u32>::decode_tlv_value_unchecked(buffer), value);

    let paired = Paired(0xDEAD_BEEF_u32);
    let mut buffer = [0u8; 6];

    let written = paired.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, 6);
    assert_eq!(buffer, [0x0A, 0x04, 0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(Paired::<u32>::decode_tlv_unchecked(buffer), paired);

    let alt: AltPaired<u32> = paired.into();
    let mut buffer = [0u8; 6];

    let written = alt.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, 6);
    assert_eq!(buffer, [0x0B, 0x04, 0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(Paired::from(AltPaired::<u32>::decode_tlv_unchecked(buffer)), paired);
}
