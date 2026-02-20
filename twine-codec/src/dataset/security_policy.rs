// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::error::TwineCodecError;

use twine_rs_macros::Tlv;

pub enum VersionThreshold {
    /// Protocol Version 1 (Thread v1.0) or 2 (Thread v1.1.x)
    ProtocolVersion2,

    /// Protocol Version 3 (Thread v1.2.x)
    ProtocolVersion3,

    /// Protocol Version 4, (Thread v1.3.x)
    ProtocolVersion4,

    /// Protocol Version 5, (Thread v1.4.x)
    ProtocolVersion5,
}

bitfield::bitfield! {
    #[derive(Clone, Copy, Eq, PartialEq, Tlv)]
    #[tlv(tlv_type = 0x0C, tlv_length = 4, derive_inner)]
    pub struct SecurityPolicy(u32);
    impl Debug;

    u16, get_rotation_time, set_rotation_time: 31, 16;

    /// Obtain Network Key bit
    ///
    /// Obtaining the Network Key for out-of-band commissioning is enabled when this bit is set (1).
    ///
    /// Default: 1 (enabled) in non-CCM networks and 0 (disabled) in CCM networks
    get_obtain_network_key_enabled, set_obtain_network_key_enabled: 15;

    /// Native Commissioner bit
    ///
    /// Native Commissioning using PSKc is allowed when this bit is set (1).
    ///
    /// Default: 1 (enabled)
    get_native_commissioning_enabled, set_native_commissioning_enabled: 14;

    /// Legacy Routers bit
    ///
    /// Thread 1.0 and 1.1.x Routers are allowed to join and operate in the network when this bit
    /// is set (1).
    ///
    /// Default: 1 (enabled) in non-CCM networks and 0 (disabled) in CCM networks
    get_legacy_routers_enabled, set_legacy_routers_enabled: 13;

    /// External Commissioner bit
    ///
    /// Indicates that External Commissioner authentication is allowed using
    /// a PSKc.
    ///
    /// Default: 1 (enabled)
    get_external_commissioner_enabled, set_external_commissioning_enabled: 12;

    /// Reserved
    ///
    /// A Thread Device must ignore this bit.
    ///
    /// Default: 0
    get_b_bit, set_b_bit: 11;

    /// Commercial Commissioning Mode bit
    ///
    /// When enabled (0), the network is indicating support for Commercial Commissioning Mode (CCM).
    ///
    /// Default: = 1 (disabled)
    get_commercial_commissioning_mode_disabled, set_commercial_commissioning_mode_disabled: 10;

    /// Autonomous Enrollment bit
    ///
    /// This bit indicates whether AE is enabled (0) or disabled (1).
    ///
    /// Default: 1 (disabled)
    get_autonomous_enrollment_disabled, set_autonomous_enrollment_disabled: 9;

    /// Network Key Provisioning bit
    ///
    /// This bit indicates whether Network Key Provisioning is enabled (0) or disabled (1).
    ///
    /// Default: 1 (disabled)
    get_network_key_provisioning_disabled, set_network_key_provisioning_disabled: 8;

    /// To BLE Link bit
    ///
    /// Default: 1
    get_to_ble_link_disabled, set_to_ble_link_disabled: 7;

    /// Non-CCM Routers bit
    ///
    /// Indicates whether, in a CCM Network, any non-CCM commissioned Thread Devices
    /// that are present in the network are allowed to operate as Routers (0) or not (1).
    ///
    /// Default: 1
    get_non_ccm_routers_disabled, set_non_ccm_routers_disabled: 6;
    u8, get_reserved_bits, set_reserved_bits: 5, 3;
    u8, get_version_threshold, set_version_threshold: 2, 0;
}

impl SecurityPolicy {
    const TYPE_NAME: &str = "SecurityPolicy";

    pub fn rotation_time_hours(&self) -> u16 {
        self.get_rotation_time()
    }

    pub fn obtain_network_key_enabled(&self) -> bool {
        self.get_obtain_network_key_enabled()
    }

    pub fn native_commissioning_enabled(&self) -> bool {
        self.get_native_commissioning_enabled()
    }

    pub fn legacy_routers_enabled(&self) -> bool {
        self.get_legacy_routers_enabled()
    }

    pub fn external_commissioner_enabled(&self) -> bool {
        self.get_external_commissioner_enabled()
    }

    pub fn commercial_commissioning_mode_enabled(&self) -> bool {
        !self.get_commercial_commissioning_mode_disabled()
    }

    pub fn autonomous_enrollment_enabled(&self) -> bool {
        !self.get_autonomous_enrollment_disabled()
    }

    pub fn network_key_provisioning_enabled(&self) -> bool {
        !self.get_network_key_provisioning_disabled()
    }

    pub fn to_ble_link_enabled(&self) -> bool {
        !self.get_to_ble_link_disabled()
    }

    pub fn non_ccm_routers_enabled(&self) -> bool {
        !self.get_non_ccm_routers_disabled()
    }

    /// Fetch the Thread Protocol Version threshold.
    ///
    /// If the protocol version is unknown, returns the value as an error.
    pub fn version_threshold(&self) -> Result<VersionThreshold, u8> {
        let threshold = self.get_version_threshold();
        let r_bit = self.get_legacy_routers_enabled();
        match (threshold, r_bit) {
            (_, true) => Ok(VersionThreshold::ProtocolVersion2),
            (0, false) => Ok(VersionThreshold::ProtocolVersion3),
            (1, false) => Ok(VersionThreshold::ProtocolVersion4),
            (2, false) => Ok(VersionThreshold::ProtocolVersion5),
            _ => Err(threshold),
        }
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        let mut policy = SecurityPolicy(0);
        policy.set_rotation_time(672);
        policy.set_obtain_network_key_enabled(true);
        policy.set_native_commissioning_enabled(true);
        policy.set_legacy_routers_enabled(true);
        policy.set_external_commissioning_enabled(true);
        policy.set_commercial_commissioning_mode_disabled(true);
        policy.set_autonomous_enrollment_disabled(true);
        policy.set_network_key_provisioning_disabled(true);
        policy.set_to_ble_link_disabled(true);
        policy.set_non_ccm_routers_disabled(true);
        policy.set_reserved_bits(0x07);
        policy.set_version_threshold(VersionThreshold::ProtocolVersion2 as u8);

        policy
    }
}

impl core::fmt::Display for SecurityPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rotation_time = self.rotation_time_hours();

        // Rotation time
        write!(f, "{} ", rotation_time)?;

        // Policy bits
        if self.obtain_network_key_enabled() {
            write!(f, "o")?;
        }

        if self.native_commissioning_enabled() {
            write!(f, "n")?;
        }

        if self.legacy_routers_enabled() {
            write!(f, "r")?;
        }

        if self.external_commissioner_enabled() {
            write!(f, "c")?;
        }

        if self.commercial_commissioning_mode_enabled() {
            write!(f, "C")?;
        }

        if self.autonomous_enrollment_enabled() {
            write!(f, "e")?;
        }

        if self.network_key_provisioning_enabled() {
            write!(f, "p")?;
        }

        if self.to_ble_link_enabled() {
            write!(f, "L")?;
        }

        if self.non_ccm_routers_enabled() {
            write!(f, "R")?;
        }

        // Raw version-threshold value
        let ver = self.get_version_threshold();
        write!(f, " {}", ver)
    }
}

#[derive(Default)]
pub struct SecurityPolicyBuilder {
    obtain_network_key_enabled: Option<bool>,
    native_commissioning_enabled: Option<bool>,
    legacy_routers_enabled: Option<bool>,
    external_commissioner_enabled: Option<bool>,
    commercial_commissioning_mode_disabled: Option<bool>,
    autonomous_enrollment_disabled: Option<bool>,
    network_key_provisioning_disabled: Option<bool>,
    to_ble_link_disabled: Option<bool>,
    non_ccm_routers_disabled: Option<bool>,
    version_threshold: Option<VersionThreshold>,
    rotation_time_hours: Option<u16>,
}

impl SecurityPolicyBuilder {
    #[cfg(test)]
    pub fn with_disabled_policy() -> Self {
        SecurityPolicyBuilder {
            obtain_network_key_enabled: Some(false),
            native_commissioning_enabled: Some(false),
            legacy_routers_enabled: Some(false),
            external_commissioner_enabled: Some(false),
            commercial_commissioning_mode_disabled: Some(true),
            autonomous_enrollment_disabled: Some(true),
            network_key_provisioning_disabled: Some(true),
            to_ble_link_disabled: Some(true),
            non_ccm_routers_disabled: Some(true),
            version_threshold: Some(VersionThreshold::ProtocolVersion3),
            rotation_time_hours: Some(672),
        }
    }

    pub fn with_default_policy() -> Self {
        SecurityPolicyBuilder {
            obtain_network_key_enabled: Some(true),
            native_commissioning_enabled: Some(true),
            legacy_routers_enabled: Some(true),
            external_commissioner_enabled: Some(true),
            commercial_commissioning_mode_disabled: Some(true),
            autonomous_enrollment_disabled: Some(true),
            network_key_provisioning_disabled: Some(true),
            to_ble_link_disabled: Some(true),
            non_ccm_routers_disabled: Some(true),
            version_threshold: Some(VersionThreshold::ProtocolVersion2),
            rotation_time_hours: Some(672),
        }
    }

    pub fn enable_obtain_network_key(mut self) -> Self {
        self.obtain_network_key_enabled = Some(true);
        self
    }

    pub fn disable_obtain_network_key(mut self) -> Self {
        self.obtain_network_key_enabled = Some(false);
        self
    }

    pub fn enable_native_commissioning(mut self) -> Self {
        self.native_commissioning_enabled = Some(true);
        self
    }

    pub fn disable_native_commissioning(mut self) -> Self {
        self.native_commissioning_enabled = Some(false);
        self
    }

    pub fn enable_legacy_routers(mut self) -> Self {
        self.legacy_routers_enabled = Some(true);
        self
    }

    pub fn disable_legacy_routers(mut self) -> Self {
        self.legacy_routers_enabled = Some(false);
        self
    }

    pub fn enable_external_commissioner(mut self) -> Self {
        self.external_commissioner_enabled = Some(true);
        self
    }

    pub fn disable_external_commissioner(mut self) -> Self {
        self.external_commissioner_enabled = Some(false);
        self
    }

    pub fn enable_commercial_commissioning(mut self) -> Self {
        self.commercial_commissioning_mode_disabled = Some(false);
        self
    }

    pub fn disable_commercial_commissioning(mut self) -> Self {
        self.commercial_commissioning_mode_disabled = Some(true);
        self
    }

    pub fn enable_autonomous_enrollment(mut self) -> Self {
        self.autonomous_enrollment_disabled = Some(false);
        self
    }

    pub fn disable_autonomous_enrollment(mut self) -> Self {
        self.autonomous_enrollment_disabled = Some(true);
        self
    }

    pub fn enable_network_key_provisioning(mut self) -> Self {
        self.network_key_provisioning_disabled = Some(false);
        self
    }

    pub fn disable_network_key_provisioning(mut self) -> Self {
        self.network_key_provisioning_disabled = Some(true);
        self
    }

    pub fn enable_to_ble_link(mut self) -> Self {
        self.to_ble_link_disabled = Some(false);
        self
    }

    pub fn disable_to_ble_link(mut self) -> Self {
        self.to_ble_link_disabled = Some(true);
        self
    }

    pub fn enable_non_ccm_routers(mut self) -> Self {
        self.non_ccm_routers_disabled = Some(false);
        self
    }

    pub fn disable_non_ccm_routers(mut self) -> Self {
        self.non_ccm_routers_disabled = Some(true);
        self
    }

    pub fn version_threshold(mut self, threshold: VersionThreshold) -> Self {
        self.version_threshold = Some(threshold);
        self
    }

    pub fn rotation_time_hours(mut self, hours: u16) -> Self {
        self.rotation_time_hours = Some(hours);
        self
    }

    pub fn build(self) -> Result<SecurityPolicy, TwineCodecError> {
        if self.obtain_network_key_enabled.is_none()
            || self.native_commissioning_enabled.is_none()
            || self.legacy_routers_enabled.is_none()
            || self.external_commissioner_enabled.is_none()
            || self.commercial_commissioning_mode_disabled.is_none()
            || self.autonomous_enrollment_disabled.is_none()
            || self.network_key_provisioning_disabled.is_none()
            || self.to_ble_link_disabled.is_none()
            || self.non_ccm_routers_disabled.is_none()
            || self.version_threshold.is_none()
            || self.rotation_time_hours.is_none()
        {
            return Err(TwineCodecError::TypeBuildError(SecurityPolicy::TYPE_NAME));
        }

        let mut policy = SecurityPolicy::default();

        if let Some(rotation_time) = self.rotation_time_hours {
            policy.set_rotation_time(rotation_time);
        }

        if let Some(o_bit) = self.obtain_network_key_enabled {
            policy.set_obtain_network_key_enabled(o_bit);
        }

        if let Some(n_bit) = self.native_commissioning_enabled {
            policy.set_native_commissioning_enabled(n_bit);
        }

        if let Some(r_bit) = self.legacy_routers_enabled {
            policy.set_legacy_routers_enabled(r_bit);
        }

        if let Some(c_bit) = self.external_commissioner_enabled {
            policy.set_external_commissioning_enabled(c_bit);
        }

        if let Some(ccm_bit) = self.commercial_commissioning_mode_disabled {
            policy.set_commercial_commissioning_mode_disabled(ccm_bit);
        }

        if let Some(ae_bit) = self.autonomous_enrollment_disabled {
            policy.set_autonomous_enrollment_disabled(ae_bit);
        }

        if let Some(np_bit) = self.network_key_provisioning_disabled {
            policy.set_network_key_provisioning_disabled(np_bit);
        }

        if let Some(ble_bit) = self.to_ble_link_disabled {
            policy.set_to_ble_link_disabled(ble_bit);
        }

        if let Some(non_ccm_bit) = self.non_ccm_routers_disabled {
            policy.set_non_ccm_routers_disabled(non_ccm_bit);
        }

        policy.set_reserved_bits(0x07);

        if let Some(version_threshold) = self.version_threshold {
            match version_threshold {
                VersionThreshold::ProtocolVersion2 => {
                    policy.set_legacy_routers_enabled(true);
                    policy.set_version_threshold(0);
                }
                VersionThreshold::ProtocolVersion3 => {
                    policy.set_legacy_routers_enabled(false);
                    policy.set_version_threshold(0);
                }
                VersionThreshold::ProtocolVersion4 => {
                    policy.set_legacy_routers_enabled(false);
                    policy.set_version_threshold(1);
                }
                VersionThreshold::ProtocolVersion5 => {
                    policy.set_legacy_routers_enabled(false);
                    policy.set_version_threshold(2);
                }
            }
        }

        Ok(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy() {
        let test = SecurityPolicyBuilder::with_default_policy()
            .build()
            .expect("Failed to build default policy");
        let expected = SecurityPolicy(0x02A0_F7F8);
        assert_eq!(expected, test);
        assert_eq!(std::format!("{}", test), "672 onrc 0");

        let test = SecurityPolicy::default();
        assert_eq!(expected, test);
        assert_eq!(std::format!("{}", test), "672 onrc 0");
    }

    #[test]
    fn disabled_policy() {
        let _ = env_logger::builder().is_test(true).try_init();
        let test = SecurityPolicyBuilder::with_disabled_policy()
            .build()
            .expect("Failed to build disabled policy");
        log::debug!("Disabled Policy: {:04x?}", test.0);

        let expected = SecurityPolicy(0x02A0_07F8);
        assert_eq!(expected, test);
    }

    #[test]
    fn o_bit() {
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_obtain_network_key()
            .build()
            .expect("Failed to build policy with O bit enabled");
        assert!(policy.obtain_network_key_enabled());
        assert_eq!(std::format!("{}", policy), "672 o 0");

        let inner = policy.0;
        assert_eq!(inner & 0x0000_8000, 0x0000_8000);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_obtain_network_key()
            .build()
            .expect("Failed to build policy with o bit enabled");
        assert!(!policy.obtain_network_key_enabled());

        let inner = policy.0;
        assert_eq!(inner & 0x0000_8000, 0);
    }

    #[test]
    fn n_bit() {
        let n_bit_mask = 0x0000_4000;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_native_commissioning()
            .build()
            .expect("Failed to build policy with n bit enabled");
        assert!(policy.native_commissioning_enabled());
        assert_eq!(std::format!("{}", policy), "672 n 0");

        let inner = policy.0;
        assert_eq!(inner & n_bit_mask, n_bit_mask);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_native_commissioning()
            .build()
            .expect("Failed to build policy with n bit disabled");
        assert!(!policy.native_commissioning_enabled());

        let inner = policy.0;
        assert_eq!(inner & n_bit_mask, 0);
    }

    #[test]
    fn r_bit() {
        let r_bit_mask = 0x0000_2000;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .version_threshold(VersionThreshold::ProtocolVersion2)
            .build()
            .expect("Failed to build policy with r bit enabled");
        assert!(policy.legacy_routers_enabled());
        assert_eq!(std::format!("{}", policy), "672 r 0");

        let inner = policy.0;
        assert_eq!(inner & r_bit_mask, r_bit_mask);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_legacy_routers()
            .build()
            .expect("Failed to build policy with r bit disabled");
        assert!(!policy.legacy_routers_enabled());

        let inner = policy.0;
        assert_eq!(inner & r_bit_mask, 0);
    }

    #[test]
    fn c_bit() {
        let c_bit_mask = 0x0000_1000;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_external_commissioner()
            .build()
            .expect("Failed to build policy with c bit enabled");
        assert!(policy.external_commissioner_enabled());
        assert_eq!(std::format!("{}", policy), "672 c 0");

        let inner = policy.0;
        assert_eq!(inner & c_bit_mask, c_bit_mask);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_external_commissioner()
            .build()
            .expect("Failed to build policy with c bit disabled");
        assert!(!policy.external_commissioner_enabled());

        let inner = policy.0;
        assert_eq!(inner & c_bit_mask, 0);
    }

    #[test]
    fn ccm_bit() {
        let ccm_bit_mask = 0x0000_0400;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_commercial_commissioning()
            .build()
            .expect("Failed to build policy with ccm bit enabled");
        assert!(policy.commercial_commissioning_mode_enabled());
        assert_eq!(std::format!("{}", policy), "672 C 0");

        let inner = policy.0;
        assert_eq!(inner & ccm_bit_mask, 0);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_commercial_commissioning()
            .build()
            .expect("Failed to build policy with ccm bit disabled");
        assert!(!policy.commercial_commissioning_mode_enabled());

        let inner = policy.0;
        assert_eq!(inner & ccm_bit_mask, ccm_bit_mask);
    }

    #[test]
    fn ae_bit() {
        let ae_bit_mask = 0x0000_0200;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_autonomous_enrollment()
            .build()
            .expect("Failed to build policy with ae bit enabled");
        assert!(policy.autonomous_enrollment_enabled());
        assert_eq!(std::format!("{}", policy), "672 e 0");

        let inner = policy.0;
        assert_eq!(inner & ae_bit_mask, 0);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_autonomous_enrollment()
            .build()
            .expect("Failed to build policy with ae bit disabled");
        assert!(!policy.autonomous_enrollment_enabled());

        let inner = policy.0;
        assert_eq!(inner & ae_bit_mask, ae_bit_mask);
    }

    #[test]
    fn np_bit() {
        let np_bit_mask = 0x0000_0100;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_network_key_provisioning()
            .build()
            .expect("Failed to build policy with np bit enabled");
        assert!(policy.network_key_provisioning_enabled());
        assert_eq!(std::format!("{}", policy), "672 p 0");

        let inner = policy.0;
        assert_eq!(inner & np_bit_mask, 0);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_network_key_provisioning()
            .build()
            .expect("Failed to build policy with np bit disabled");
        assert!(!policy.network_key_provisioning_enabled());

        let inner = policy.0;
        assert_eq!(inner & np_bit_mask, np_bit_mask);
    }

    #[test]
    fn ncr_bit() {
        let ncr_bit_mask = 0x0000_0040;
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .enable_non_ccm_routers()
            .build()
            .expect("Failed to build policy with ncr bit enabled");
        assert!(policy.non_ccm_routers_enabled());
        assert_eq!(std::format!("{}", policy), "672 R 0");

        let inner = policy.0;
        assert_eq!(inner & ncr_bit_mask, 0);

        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .disable_non_ccm_routers()
            .build()
            .expect("Failed to build policy with ncr bit disabled");
        assert!(!policy.non_ccm_routers_enabled());

        let inner = policy.0;
        assert_eq!(inner & ncr_bit_mask, ncr_bit_mask);
    }

    #[test]
    fn version_threshold() {
        let policy = SecurityPolicyBuilder::with_disabled_policy()
            .version_threshold(VersionThreshold::ProtocolVersion4)
            .build()
            .expect("Failed to build policy with Protocol Version 4 threshold");
        assert!(!policy.legacy_routers_enabled());

        assert!(matches!(
            policy.version_threshold(),
            Ok(VersionThreshold::ProtocolVersion4)
        ));

        assert_eq!(std::format!("{}", policy), "672  1");
        assert_eq!(policy.0 & 0x0000_0003, 1);
    }

    #[test]
    fn display_security_policy() {
        let _ = env_logger::builder().is_test(true).try_init();

        let policy = SecurityPolicy::default();
        assert_eq!(std::format!("{}", policy), "672 onrc 0");

        let policy = SecurityPolicyBuilder::default()
            .disable_obtain_network_key()
            .disable_native_commissioning()
            .disable_legacy_routers()
            .disable_external_commissioner()
            .enable_commercial_commissioning()
            .disable_autonomous_enrollment()
            .disable_network_key_provisioning()
            .disable_to_ble_link()
            .disable_non_ccm_routers()
            .version_threshold(VersionThreshold::ProtocolVersion4)
            .rotation_time_hours(672)
            .build()
            .inspect_err(|e| log::trace!("Failed to build security policy: {e:?}"));

        let policy = policy.expect("Failed to build security policy");

        log::debug!("Policy: {policy:?}");
        assert!(!policy.obtain_network_key_enabled());
        assert_eq!(std::format!("{}", policy), "672 C 1");
    }
}
