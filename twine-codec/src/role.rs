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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_disabled() {
        assert!(matches!(NetworkRole::default(), NetworkRole::Disabled));
    }

    #[test]
    fn parse_all_variants_lowercase() {
        assert!(matches!(
            "disabled".parse::<NetworkRole>().unwrap(),
            NetworkRole::Disabled
        ));
        assert!(matches!(
            "detached".parse::<NetworkRole>().unwrap(),
            NetworkRole::Detached
        ));
        assert!(matches!(
            "child".parse::<NetworkRole>().unwrap(),
            NetworkRole::Child
        ));
        assert!(matches!(
            "router".parse::<NetworkRole>().unwrap(),
            NetworkRole::Router
        ));
        assert!(matches!(
            "leader".parse::<NetworkRole>().unwrap(),
            NetworkRole::Leader
        ));
    }

    #[test]
    fn parse_case_insensitive() {
        assert!(matches!(
            "DISABLED".parse::<NetworkRole>().unwrap(),
            NetworkRole::Disabled
        ));
        assert!(matches!(
            "Router".parse::<NetworkRole>().unwrap(),
            NetworkRole::Router
        ));
        assert!(matches!(
            "LEADER".parse::<NetworkRole>().unwrap(),
            NetworkRole::Leader
        ));
    }

    #[test]
    fn parse_invalid() {
        assert!("unknown".parse::<NetworkRole>().is_err());
        assert!("".parse::<NetworkRole>().is_err());
    }
}
