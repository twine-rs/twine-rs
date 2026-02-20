// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use twine_rs_macros::Tlv;
use twine_tlv::prelude::*;

use crate::error::TwineCodecError;

const NETWORK_NAME_MAX_SIZE: usize = 16;

/// A human readable UTF-8 string to identify the Thread network.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x03, derive_inner)]
pub struct NetworkName([u8; NETWORK_NAME_MAX_SIZE + 1]);

impl TlvLength for NetworkName {
    fn tlv_len(&self) -> usize {
        self.0
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(NETWORK_NAME_MAX_SIZE)
    }
}

impl core::fmt::Display for NetworkName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut length = 0;

        for byte in self.0.iter() {
            if *byte == 0 {
                break;
            }
            length += 1;
        }

        let s = core::str::from_utf8(&self.0[..length]).map_err(|_| core::fmt::Error)?;
        write!(f, "{}", s)
    }
}

impl FromStr for NetworkName {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.as_bytes();
        let length = raw.len();

        if length > NETWORK_NAME_MAX_SIZE {
            return Err(TwineCodecError::BufferMaxLength(
                NETWORK_NAME_MAX_SIZE,
                length,
            ));
        }

        let mut n = [0_u8; NETWORK_NAME_MAX_SIZE + 1];
        raw.iter().enumerate().for_each(|(i, byte)| {
            n[i] = *byte;
        });

        Ok(Self(n))
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    const EXPECTED_NAME_STR: &str = "TestNetwork";

    #[test]
    fn success_display_network_name() {
        let network_name = NetworkName::from_str(EXPECTED_NAME_STR).unwrap();
        let name_str = format!("{}", network_name);
        assert_eq!(name_str, EXPECTED_NAME_STR);
    }

    #[test]
    fn success_tlv_len() {
        let network_name = NetworkName::from_str(EXPECTED_NAME_STR).unwrap();
        assert_eq!(network_name.tlv_len(), EXPECTED_NAME_STR.len());
    }

    #[test]
    fn success_from_str() {
        let network_name = NetworkName::from_str(EXPECTED_NAME_STR).unwrap();
        let name_str = format!("{}", network_name);
        assert_eq!(name_str, EXPECTED_NAME_STR);
    }
}
