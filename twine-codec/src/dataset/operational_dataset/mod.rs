// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::str::FromStr;

use twine_tlv::TlvCollection;

use crate::{
    dataset::{
        timestamp::{ActiveTimestamp, PendingTimestamp},
        DelayTimer, ExtendedPanId, MeshLocalPrefix, NetworkKey, NetworkName, Pskc, SecurityPolicy,
        Timestamp,
    },
    radio::{Channel, ChannelMask, PanId},
    TwineCodecError,
};

mod iter;
pub use iter::OperationalDatasetIter;

const OPERATIONAL_DATASET_MAX_SIZE: usize = 256;

macro_rules! decode_type {
    ($name:ident, $decode_type:ty) => {
        pub fn $name(&self) -> Option<$decode_type> {
            self.collection.decode_type_unchecked::<$decode_type>()
        }
    };
}

#[derive(Debug)]
pub struct OperationalDataset {
    collection: TlvCollection<OPERATIONAL_DATASET_MAX_SIZE>,
}

impl OperationalDataset {
    /// Generate a new random Active Operational Dataset
    #[cfg(any(test, feature = "std"))]
    pub fn random() -> Result<Self, TwineCodecError> {
        let mut collection = TlvCollection::default();

        use crate::dataset::timestamp::Authoritative;
        let active_timestamp = Timestamp::now(Authoritative(false));
        let _ = collection.push(ActiveTimestamp::from(active_timestamp))?;

        let channel = Channel::random();
        let _ = collection.push(channel)?;

        // todo: wake_up_channel

        let channel_mask = ChannelMask::default();
        let _ = collection.push(channel_mask)?;

        let xpan = ExtendedPanId::random();
        let _ = collection.push(ExtendedPanId::from(xpan))?;

        let mesh_local_prefix = MeshLocalPrefix::random_ula();
        let _ = collection.push(mesh_local_prefix)?;

        let network_key = NetworkKey::random();
        let _ = collection.push(network_key)?;

        let pan_id = PanId::random();
        let network_name = alloc::format!("Twine-{:x}", pan_id.get());
        let _ = collection.push(NetworkName::from_str(&network_name)?)?;

        let _ = collection.push(pan_id)?;

        let pskc = Pskc::random();
        let _ = collection.push(pskc)?;

        let security_policy = SecurityPolicy::default();
        let _ = collection.push(security_policy)?;

        Ok(Self { collection })
    }

    pub fn active_timestamp(&self) -> Option<Timestamp> {
        self.collection
            .decode_type_unchecked::<ActiveTimestamp>()
            .map(Timestamp::from)
    }

    pub fn set_active_timestamp(&mut self, timestamp: Timestamp) -> Result<(), TwineCodecError> {
        let active_timestamp = ActiveTimestamp::from(timestamp);
        self.collection.replace_or_push(active_timestamp)?;
        Ok(())
    }

    pub fn pending_timestamp(&self) -> Option<Timestamp> {
        self.collection
            .decode_type_unchecked::<PendingTimestamp>()
            .map(Timestamp::from)
    }

    decode_type!(delay_timer, DelayTimer);
    decode_type!(channel, Channel);
    // todo: wake_up_channel
    decode_type!(pan_id, PanId);
    decode_type!(channel_mask, ChannelMask);
    decode_type!(extended_pan_id, ExtendedPanId);
    decode_type!(network_name, NetworkName);
    decode_type!(pskc, Pskc);
    decode_type!(network_key, NetworkKey);
    decode_type!(mesh_local_prefix, MeshLocalPrefix);
    decode_type!(security_policy, SecurityPolicy);

    #[cfg(any(test, feature = "std"))]
    pub fn pretty_fmt(&self) {
        std::println!("Operational Dataset: {:?}", self);
        self.iter().for_each(|item| std::println!("{item:?}"));
    }

    pub fn iter(&self) -> OperationalDatasetIter<'_> {
        OperationalDatasetIter {
            inner: (&self.collection).into_iter(),
        }
    }

    #[cfg(any(test, feature = "alloc"))]
    pub fn as_hex_string(&self) -> alloc::string::String {
        let mut hex_string = alloc::string::String::new();
        for tlv in &self.collection {
            hex_string.push_str(&hex::encode(tlv));
        }
        hex_string
    }
}

impl FromStr for OperationalDataset {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure even number of characters
        if (s.len() & 1) != 0 {
            return Err(TwineCodecError::HexDecodeError);
        }

        let n = s.len() / 2;
        let mut buffer = [0_u8; OPERATIONAL_DATASET_MAX_SIZE];

        // Ensure buffer is large enough
        if n > buffer.len() {
            return Err(TwineCodecError::HexDecodeError);
        }

        hex::decode_to_slice(s, &mut buffer[..n]).map_err(|_| TwineCodecError::HexDecodeError)?;
        let collection = TlvCollection::new_from_static(buffer);

        Ok(Self { collection })
    }
}

impl core::fmt::Display for OperationalDataset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iter()
            .fold(Ok(()), |res, item| res.and_then(|_| writeln!(f, "{item}")))
    }
}

#[cfg(test)]
mod tests {
    use crate::{dataset::timestamp::Authoritative, SecurityPolicyBuilder};

    use super::*;

    #[test]
    fn success_from_str() {
        let dataset_str = "0e080000000000010000000300000c4a0300001335060004001fffe002081bb896bef533a5850708fd48b2e8c34e7dc70510e9b948988752752873570d09ada4d0be030f4f70656e5468726561642d623364650102b3de0410f9f07ed37fbb6828fb3b26b63bdea3c30c0402a0f7f8";
        let dataset = OperationalDataset::from_str(dataset_str).unwrap();

        let active_timestamp = dataset.active_timestamp().unwrap();
        let channel = dataset.channel().unwrap();
        // wake_up_channel
        // channel_mask
        let xpan = dataset.extended_pan_id().unwrap();
        let mesh_local_prefix: MeshLocalPrefix = dataset.mesh_local_prefix().unwrap();
        let network_key = dataset.network_key().unwrap();
        let network_name = dataset.network_name().unwrap();
        let pan_id = dataset.pan_id().unwrap();
        let pskc = dataset.pskc().unwrap();
        let security_policy = dataset.security_policy().unwrap();

        assert_eq!(
            active_timestamp,
            Timestamp::from((1, 1, Authoritative(false)))
        );
        assert_eq!(channel, Channel::new(0, 12));
        // wake up channel
        // channel mask
        assert_eq!(
            xpan,
            ExtendedPanId::from([0x1b, 0xb8, 0x96, 0xbe, 0xf5, 0x33, 0xa5, 0x85])
        );
        assert_eq!(
            mesh_local_prefix,
            MeshLocalPrefix::from([0xfd, 0x48, 0xb2, 0xe8, 0xc3, 0x4e, 0x7d, 0xc7])
        );
        assert_eq!(
            network_key,
            NetworkKey::from(u128::from_be_bytes([
                0xe9, 0xb9, 0x48, 0x98, 0x87, 0x52, 0x75, 0x28, 0x73, 0x57, 0x0d, 0x09, 0xad, 0xa4,
                0xd0, 0xbe
            ]))
        );
        assert_eq!(
            network_name,
            NetworkName::from_str("OpenThread-b3de").unwrap()
        );
        assert_eq!(pan_id, PanId::from(0xb3de));
        assert_eq!(
            pskc,
            Pskc::from([
                0xf9, 0xf0, 0x7e, 0xd3, 0x7f, 0xbb, 0x68, 0x28, 0xfb, 0x3b, 0x26, 0xb6, 0x3b, 0xde,
                0xa3, 0xc3
            ])
        );
        assert_eq!(
            security_policy,
            SecurityPolicyBuilder::with_default_policy()
                .build()
                .unwrap()
        );
    }
}
