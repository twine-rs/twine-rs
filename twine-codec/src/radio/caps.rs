// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

bitflags::bitflags! {
    /// Radio capabilities
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct RadioCapabilities: u8 {
        /// Radio supports no capability
        const NONE = 0b0000_0000;

        /// Radio supports AckTime event
        const ACK_TIMEOUT = 0b0000_0001;

        /// Radio supports Energy Scans
        const ENERGY_SCAN = 0b0000_0010;

        /// Radio supports TX retry logic with collision avoidance (CSMA)
        const TRANSMIT_RETRIES = 0b0000_0100;

        /// Radio supports CSMA backoff for frame transmission (but no retry)
        const CSMA_BACKOFF = 0b0000_1000;

        /// Radio supports TX security
        const SLEEP_TO_TX = 0b0001_0000;

        /// Radio supports TX security
        const TRANSMIT_SEC = 0b0010_0000;

        /// Radio supports TX at specific time
        const TRANSMIT_TIMING = 0b0100_0000;

        /// Radio supports RX at specific time
        const RECEIVE_TIMING = 0b1000_0000;
    }
}

impl RadioCapabilities {
    /// Check if radio supports no capability
    pub fn none(&self) -> bool {
        self.is_empty()
    }

    /// Check if radio supports AckTime event
    pub fn ack_timeout(&self) -> bool {
        self.contains(Self::ACK_TIMEOUT)
    }

    /// Check if radio supports Energy Scans
    pub fn energy_scan(&self) -> bool {
        self.contains(Self::ENERGY_SCAN)
    }

    /// Check if radio supports transmit retries
    pub fn transmit_retries(&self) -> bool {
        self.contains(Self::TRANSMIT_RETRIES)
    }

    /// Check if radio supports CSMA backoff
    pub fn csma_backoff(&self) -> bool {
        self.contains(Self::CSMA_BACKOFF)
    }

    /// Check if radio supports sleep to TX
    pub fn sleep_to_tx(&self) -> bool {
        self.contains(Self::SLEEP_TO_TX)
    }

    /// Check if radio supports transmit security
    pub fn transmit_security(&self) -> bool {
        self.contains(Self::TRANSMIT_SEC)
    }

    /// Check if radio supports transmit timing
    pub fn transmit_timing(&self) -> bool {
        self.contains(Self::TRANSMIT_TIMING)
    }

    /// Check if radio supports receive timing
    pub fn receive_timing(&self) -> bool {
        self.contains(Self::RECEIVE_TIMING)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    static ALL_CAPS: LazyLock<RadioCapabilities> = LazyLock::new(|| {
        RadioCapabilities::ACK_TIMEOUT
            | RadioCapabilities::ENERGY_SCAN
            | RadioCapabilities::TRANSMIT_RETRIES
            | RadioCapabilities::CSMA_BACKOFF
            | RadioCapabilities::SLEEP_TO_TX
            | RadioCapabilities::TRANSMIT_SEC
            | RadioCapabilities::TRANSMIT_TIMING
            | RadioCapabilities::RECEIVE_TIMING
    });

    #[test]
    fn contains_none() {
        let caps = RadioCapabilities::NONE;
        assert!(caps.none());

        assert!(!ALL_CAPS.none());
    }

    #[test]
    fn contains_ack_timeout() {
        let caps = RadioCapabilities::ACK_TIMEOUT;
        assert!(!caps.none());
        assert!(caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.ack_timeout());
    }

    #[test]
    fn contains_energy_scan() {
        let caps = RadioCapabilities::ENERGY_SCAN;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.energy_scan());
    }

    #[test]
    fn contains_transmit_retries() {
        let caps = RadioCapabilities::TRANSMIT_RETRIES;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.transmit_retries());
    }

    #[test]
    fn contains_csma_backoff() {
        let caps = RadioCapabilities::CSMA_BACKOFF;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.csma_backoff());
    }

    #[test]
    fn contains_sleep_to_tx() {
        let caps = RadioCapabilities::SLEEP_TO_TX;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.sleep_to_tx());
    }

    #[test]
    fn contains_transmit_security() {
        let caps = RadioCapabilities::TRANSMIT_SEC;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.transmit_security());
    }

    #[test]
    fn contains_transmit_timing() {
        let caps = RadioCapabilities::TRANSMIT_TIMING;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(caps.transmit_timing());
        assert!(!caps.receive_timing());

        assert!(ALL_CAPS.transmit_timing());
    }

    #[test]
    fn contains_receive_timing() {
        let caps = RadioCapabilities::RECEIVE_TIMING;
        assert!(!caps.none());
        assert!(!caps.ack_timeout());
        assert!(!caps.energy_scan());
        assert!(!caps.transmit_retries());
        assert!(!caps.csma_backoff());
        assert!(!caps.sleep_to_tx());
        assert!(!caps.transmit_security());
        assert!(!caps.transmit_timing());
        assert!(caps.receive_timing());

        assert!(ALL_CAPS.receive_timing());
    }
}
