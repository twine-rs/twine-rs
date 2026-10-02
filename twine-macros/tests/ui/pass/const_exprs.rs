// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;
use twine_tlv::{DecodeTlvUnchecked as _, TlvConstantMetadata, TlvType, TryEncodeTlv as _};

const EXAMPLE_TLV_TYPE: u8 = 0x02;
const EXAMPLE_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = EXAMPLE_TLV_TYPE, tlv_length = EXAMPLE_SIZE, derive_inner)]
struct Example([u8; EXAMPLE_SIZE]);

fn main() {
    assert_eq!(Example::TLV_TYPE, EXAMPLE_TLV_TYPE);
    assert_eq!(Example::TLV_LEN, EXAMPLE_SIZE);

    let value = Example([0xAA; EXAMPLE_SIZE]);
    let mut buffer = [0u8; EXAMPLE_SIZE + 2];

    let written = value.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, EXAMPLE_SIZE + 2);
    assert_eq!(buffer[0], EXAMPLE_TLV_TYPE);
    assert_eq!(buffer[1], EXAMPLE_SIZE as u8);

    assert_eq!(Example::decode_tlv_unchecked(buffer), value);
}
