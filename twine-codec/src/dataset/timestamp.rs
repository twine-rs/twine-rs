// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::Buf;

use twine_macros::Tlv;
use twine_tlv::{
    write_tlv, DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, TlvConstantMetadata,
    TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TwineTlvError,
};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Authoritative(pub bool);

impl Authoritative {
    pub(crate) fn is_authoritative(&self) -> bool {
        self.0
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Tlv)]
#[tlv(variant = "Active", tlv_type = 0x0e, tlv_length = 8, derive_inner)]
#[tlv(variant = "Pending", tlv_type = 0x33, tlv_length = 8, derive_inner)]
pub struct Timestamp(u64);

impl Timestamp {
    #[cfg(any(test, feature = "std"))]
    pub fn now(auth: Authoritative) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now();
        let seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

        Self::from((seconds, 0, auth))
    }

    pub fn seconds(&self) -> u64 {
        self.0 >> 16
    }

    pub fn ticks(&self) -> u16 {
        ((self.0 >> 1) & 0x7fff) as u16
    }

    pub fn is_authoritative(&self) -> bool {
        (self.0 & 0x1) != 0
    }
}
impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.seconds(),)
    }
}

impl core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Timestamp {{ seconds: {}, ticks: {}, authoritative: {} }}",
            self.seconds(),
            self.ticks(),
            self.is_authoritative()
        )
    }
}

impl From<(u64, u16, Authoritative)> for Timestamp {
    fn from(parts: (u64, u16, Authoritative)) -> Self {
        let (seconds, ticks, auth) = parts;
        let seconds = seconds << 16;
        let ticks = (ticks as u64) & 0xfffe;
        let auth = if auth.is_authoritative() { 1u64 } else { 0u64 };

        Self(seconds | ticks | auth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use twine_tlv::TryEncodeTlv;

    #[test]
    fn success_from_parts() {
        let timestamp = Timestamp::from((0x1234_5678, 0x9abc, Authoritative(true)));
        assert_eq!(timestamp.0, 0x1234_5678_9abd_u64);

        assert_eq!(timestamp.seconds(), 0x1234_5678);
        assert_eq!(timestamp.ticks(), 0x9abc >> 1);
        assert_eq!(timestamp.is_authoritative(), true);
    }

    #[test]
    fn success_active_timestamp_to_tlv() {
        let timestamp = Timestamp::from((0x1234_5678, 0x9abc, Authoritative(true)));
        let timestamp = ActiveTimestamp::from(timestamp);

        let mut buffer = [0u8; 2 + 8];
        let written = timestamp.try_encode_tlv(&mut buffer).unwrap();

        assert_eq!(written, 10);
        assert_eq!(buffer[0], 0x0e); // TLV Type
        assert_eq!(buffer[1], 0x08); // TLV Length
        assert_eq!(&buffer[2..], &0x1234_5678_9abd_u64.to_be_bytes()[..]);
    }

    #[test]
    fn success_pending_timestamp_to_tlv() {
        let timestamp = Timestamp::from((0x1234_5678, 0x9abc, Authoritative(true)));
        let timestamp = PendingTimestamp::from(timestamp);

        let mut buffer = [0u8; 2 + 8];
        let written = timestamp.try_encode_tlv(&mut buffer).unwrap();

        assert_eq!(written, 10);
        assert_eq!(buffer[0], 0x33); // TLV Type
        assert_eq!(buffer[1], 0x08); // TLV Length
        assert_eq!(&buffer[2..], &0x1234_5678_9abd_u64.to_be_bytes()[..]);
    }
}
