// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

mod dataset;
mod error;
mod radio;
mod rloc16;
mod role;
mod util;

pub use dataset::{
    decode_operational_dataset_item, ActiveTimestamp, Authoritative, Components, DelayTimer,
    ExtendedPanId, MeshLocalPrefix, NetworkKey, NetworkName, OperationalDataset,
    OperationalDatasetItem, PendingTimestamp, Pskc, SecurityPolicy, SecurityPolicyBuilder,
    Timestamp, VersionThreshold,
};
pub use error::TwineCodecError;
pub use radio::{
    Channel, ChannelMask, ChannelPageMask, Eui64, ExtendedAddress, PanId, RadioCapabilities,
};
pub use rloc16::Rloc16;
pub use role::NetworkRole;
#[cfg(any(test, feature = "alloc"))]
pub use twine_tlv::TlvCollectionDiffEntry;
pub(crate) use util::{fill_random_bytes, random_range_u16};
