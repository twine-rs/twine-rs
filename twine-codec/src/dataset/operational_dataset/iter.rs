// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use twine_tlv::{DecodeTlvUnchecked, TlvCollection, TlvType};

use crate::{
    dataset::{
        timestamp::{ActiveTimestamp, PendingTimestamp},
        DelayTimer, ExtendedPanId, MeshLocalPrefix, NetworkKey, NetworkName, Pskc, SecurityPolicy,
    },
    radio::{Channel, PanId},
    ChannelMask,
};

use super::OPERATIONAL_DATASET_MAX_SIZE;

macro_rules! decode_dataset_tlv_unchecked {
    ($tlv:expr, {
        $(
            $ty:ty => $variant:ident
        ),* $(,)?
    }) => {{
        let tlv = $tlv;
        let t = tlv[0];

        match t {
            $(
                <$ty>::TLV_TYPE => {
                    let value = <$ty>::decode_tlv_unchecked(tlv);
                    OperationalDatasetItem::$variant(value)
                }
            )*
            _ => OperationalDatasetItem::Unknown(tlv),
        }
    }};
}

#[derive(Debug)]
pub enum OperationalDatasetItem<'a> {
    ActiveTimestamp(ActiveTimestamp),
    PendingTimestamp(PendingTimestamp),
    DelayTimer(DelayTimer),
    Channel(Channel),
    ChannelMask(ChannelMask),
    ExtendedPanId(ExtendedPanId),
    MeshLocalPrefix(MeshLocalPrefix),
    NetworkKey(NetworkKey),
    NetworkName(NetworkName),
    PanId(PanId),
    Pskc(Pskc),
    SecurityPolicy(SecurityPolicy),
    Unknown(&'a [u8]),
}

impl core::fmt::Display for OperationalDatasetItem<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OperationalDatasetItem::ActiveTimestamp(v) => {
                write!(f, "Active Timestamp: {}", v.seconds())
            }
            OperationalDatasetItem::PendingTimestamp(v) => {
                write!(f, "Pending Timestamp: {}", v.seconds())
            }
            OperationalDatasetItem::DelayTimer(v) => write!(f, "Delay Timer: {v:?}"),
            OperationalDatasetItem::Channel(v) => write!(f, "Channel: {}", v.channel()),
            // todo: wake up channel
            OperationalDatasetItem::ChannelMask(v) => write!(f, "Channel Mask: {v}"),
            OperationalDatasetItem::ExtendedPanId(v) => write!(f, "Ext PAN ID: {v}"),
            OperationalDatasetItem::MeshLocalPrefix(v) => write!(f, "Mesh Local Prefix: {v}"),
            OperationalDatasetItem::NetworkKey(v) => write!(f, "Network Key: {v}"),
            OperationalDatasetItem::NetworkName(v) => write!(f, "Network Name: {v}"),
            OperationalDatasetItem::PanId(v) => write!(f, "PAN ID: {v}"),
            OperationalDatasetItem::Pskc(v) => write!(f, "PSKc: {v}"),
            OperationalDatasetItem::SecurityPolicy(v) => write!(f, "Security Policy: {v}"),
            OperationalDatasetItem::Unknown(tlv) => write!(f, "Unknown TLV: {tlv:x?}"),
        }
    }
}

pub struct OperationalDatasetIter<'a> {
    pub(crate) inner: <&'a TlvCollection<OPERATIONAL_DATASET_MAX_SIZE> as IntoIterator>::IntoIter,
}

impl<'a> Iterator for OperationalDatasetIter<'a> {
    type Item = OperationalDatasetItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let tlv = self.inner.next()?;
        Some(decode_operational_dataset_item(tlv))
    }
}

fn decode_operational_dataset_item(tlv: &[u8]) -> OperationalDatasetItem<'_> {
    decode_dataset_tlv_unchecked!(tlv, {
        ActiveTimestamp  => ActiveTimestamp,
        PendingTimestamp => PendingTimestamp,
        DelayTimer       => DelayTimer,
        Channel          => Channel,
        ChannelMask      => ChannelMask,
        ExtendedPanId    => ExtendedPanId,
        MeshLocalPrefix  => MeshLocalPrefix,
        NetworkKey       => NetworkKey,
        NetworkName      => NetworkName,
        PanId            => PanId,
        Pskc             => Pskc,
        SecurityPolicy   => SecurityPolicy,
    })
}
