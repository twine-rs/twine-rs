// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use crate::error::TwineCodecError;

pub struct Rloc16(u16);

impl From<Rloc16> for u16 {
    fn from(value: Rloc16) -> Self {
        value.0
    }
}

impl From<u16> for Rloc16 {
    fn from(rloc16: u16) -> Self {
        Self(rloc16)
    }
}

impl FromStr for Rloc16 {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u16::from_str_radix(s, 16).map_err(|_| TwineCodecError::StringParseError)?;
        Ok(Self(value))
    }
}

impl core::fmt::Display for Rloc16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}
