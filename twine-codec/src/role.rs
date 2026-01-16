// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use crate::error::TwineCodecError;

#[derive(Debug, Default)]
pub enum NetworkRole {
    #[default]
    Disabled,
    Detached,
    Child,
    Router,
    Leader,
}

impl FromStr for NetworkRole {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "disabled" => Ok(NetworkRole::Disabled),
            "detached" => Ok(NetworkRole::Detached),
            "child" => Ok(NetworkRole::Child),
            "router" => Ok(NetworkRole::Router),
            "leader" => Ok(NetworkRole::Leader),
            _ => Err(TwineCodecError::StringParseError),
        }
    }
}
