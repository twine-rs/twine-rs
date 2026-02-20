// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_rs_macros::Tlv;

const MESH_LOCAL_PREFIX_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x07, tlv_length = 8, derive_inner)]
pub struct MeshLocalPrefix([u8; MESH_LOCAL_PREFIX_SIZE]);

impl MeshLocalPrefix {
    pub fn random_ula() -> Self {
        let mut bytes = [0u8; MESH_LOCAL_PREFIX_SIZE];
        bytes[0] = 0xfd; // ULA prefix
        crate::fill_random_bytes(&mut bytes[1..]);
        Self(bytes)
    }
}

impl core::fmt::Display for MeshLocalPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let b = &self.0;

        let g0 = ((b[0] as u16) << 8) | (b[1] as u16);
        let g1 = ((b[2] as u16) << 8) | (b[3] as u16);
        let g2 = ((b[4] as u16) << 8) | (b[5] as u16);
        let g3 = ((b[6] as u16) << 8) | (b[7] as u16);

        write!(f, "{g0:04x}:{g1:04x}:{g2:04x}:{g3:04x}::/64")
    }
}

impl From<[u8; MESH_LOCAL_PREFIX_SIZE]> for MeshLocalPrefix {
    fn from(value: [u8; MESH_LOCAL_PREFIX_SIZE]) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_mesh_local_prefix() {
        let bytes = [0xfd, 0xe2, 0x2f, 0xdc, 0x94, 0x77, 0x9b, 0x16];
        let prefix = MeshLocalPrefix::from(bytes);
        assert_eq!(std::format!("{}", prefix), "fde2:2fdc:9477:9b16::/64");
    }
}
