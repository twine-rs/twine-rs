// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::{Tlv, TlvTransparent};
use twine_tlv::{DecodeTlvUnchecked as _, TryEncodeTlv as _};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv, TlvTransparent)]
#[tlv(tlv_type = 0x07, tlv_length = 4)]
struct Example(u32);

fn main() {
    let value = Example(0xDEAD_BEEF);
    let mut buffer = [0u8; 6];

    let written = value.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, 6);
    assert_eq!(buffer, [0x07, 0x04, 0xDE, 0xAD, 0xBE, 0xEF]);

    assert_eq!(Example::decode_tlv_unchecked(buffer), value);
}
