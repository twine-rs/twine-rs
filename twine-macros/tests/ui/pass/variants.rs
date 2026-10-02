// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;
use twine_tlv::{DecodeTlvUnchecked as _, TlvType, TryEncodeTlv as _};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(
    variants = [("Active", tlv_type = 0x0e), ("Pending", tlv_type = 0x33)],
    tlv_length = 8,
    derive_inner
)]
struct Timestamp(u64);

fn main() {
    assert_eq!(Timestamp::TLV_TYPE, 0x0e);
    assert_eq!(ActiveTimestamp::TLV_TYPE, 0x0e);
    assert_eq!(PendingTimestamp::TLV_TYPE, 0x33);

    let pending: PendingTimestamp = Timestamp(7).into();
    let mut buffer = [0u8; 10];

    let written = pending.try_encode_tlv(&mut buffer).expect("encode");
    assert_eq!(written, 10);
    assert_eq!(buffer[0], 0x33);
    assert_eq!(buffer[1], 8);

    let decoded = PendingTimestamp::decode_tlv_unchecked(buffer);
    assert_eq!(Timestamp::from(decoded), Timestamp(7));
}
