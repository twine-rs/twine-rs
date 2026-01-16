// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

use twine_macros::Tlv;
use twine_tlv::prelude::*;

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
