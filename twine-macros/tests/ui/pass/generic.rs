// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;
use twine_tlv::{DecodeTlvValueUnchecked, TryEncodeTlvValue, TwineTlvError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x05, tlv_length = 2, variants = [("Alt", tlv_type = 0x06)])]
struct Pair<T: Copy>(T)
where
    T: Default;

impl<T: Copy> DecodeTlvValueUnchecked for Pair<T>
where
    T: Default,
{
    fn decode_tlv_value_unchecked(_buffer: impl AsRef<[u8]>) -> Self {
        Pair(T::default())
    }
}

impl<T: Copy> TryEncodeTlvValue for Pair<T>
where
    T: Default,
{
    fn try_encode_tlv_value(&self, _buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        Ok(0)
    }
}

fn main() {
    let alt: AltPair<u16> = Pair(1u16).into();
    assert_eq!(Pair::from(alt), Pair(1u16));
}
