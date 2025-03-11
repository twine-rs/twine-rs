use crate::error::TwineCodecError;

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
    #[derive(Clone, Copy, Default, Eq, PartialEq)]
    pub struct SecurityPolicy(u32);
    impl Debug;

    u16, get_rotation_time, set_rotation_time: 31, 16;
    get_obtain_network_key_enabled, set_obtain_network_key_enabled: 15;
    get_native_commissioning_enabled, set_native_commissioning_enabled: 14;
    get_legacy_routers_enabled, set_legacy_routers_enabled: 13;
    get_external_commissioner_enabled, set_external_commissioning_enabled: 12;
    get_b_bit, set_b_bit: 11;
    get_commercial_commissioning_mode_disabled, set_commercial_commissioning_mode_disabled: 10;
    get_autonomous_enrollment_disabled, set_autonomous_enrollment_disabled: 9;
    get_network_key_provisioning_disabled, set_network_key_provisioning_disabled: 8;
    get_to_ble_link_disabled, set_to_ble_link_disabled: 7;
    get_non_ccm_routers_disabled, set_non_ccm_routers_disabled: 6;
    u8, get_reserved_bits, set_reserved_bits: 5, 3;
    u8, get_version_threshold, set_version_threshold: 2, 0;
}

impl SecurityPolicy {
    fn type_name() -> &'static str {
        "SecurityPolicy"
    }

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

    pub fn commercial_commissioning_mode_disabled(&self) -> bool {
        self.get_commercial_commissioning_mode_disabled()
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

pub struct SecurityPolicyBuilder {
    obtain_network_key_enabled: Option<bool>,
    native_commissioning_enabled: Option<bool>,
    legacy_routers_enabled: Option<bool>,
    external_commissioner_enabled: Option<bool>,
    commercial_commissioning_mode_disabled: Option<bool>,
    autonomous_enrollment_disabled: Option<bool>,
    network_key_provisioning_disabled: Option<bool>,
    to_ble_link_enabled: Option<bool>,
    non_ccm_routers_disabled: Option<bool>,
    version_threshold: Option<VersionThreshold>,
    rotation_time_hours: Option<u16>,
}

impl SecurityPolicyBuilder {
    pub fn with_default_policy() -> Self {
        SecurityPolicyBuilder {
            obtain_network_key_enabled: Some(true),
            native_commissioning_enabled: Some(true),
            legacy_routers_enabled: Some(true),
            external_commissioner_enabled: Some(true),
            commercial_commissioning_mode_disabled: Some(true),
            autonomous_enrollment_disabled: Some(true),
            network_key_provisioning_disabled: Some(true),
            to_ble_link_enabled: Some(true),
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

    pub fn enable_non_ccm_routers(mut self) -> Self {
        self.non_ccm_routers_disabled = Some(false);
        self
    }

    pub fn disable_non_ccm_routers(mut self) -> Self {
        self.non_ccm_routers_disabled = Some(true);
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
            || self.to_ble_link_enabled.is_none()
            || self.non_ccm_routers_disabled.is_none()
            || self.version_threshold.is_none()
            || self.rotation_time_hours.is_none()
        {
            return Err(TwineCodecError::TypeBuildError(SecurityPolicy::type_name()));
        }

        let mut policy = SecurityPolicy::default();

        if let Some(o_bit) = self.obtain_network_key_enabled {
            if o_bit {
                policy.set_obtain_network_key_enabled(true);
            }
        }

        if let Some(n_bit) = self.native_commissioning_enabled {
            if n_bit {
                policy.set_native_commissioning_enabled(true);
            }
        }

        if let Some(r_bit) = self.legacy_routers_enabled {
            if r_bit {
                policy.set_legacy_routers_enabled(true);
            }
        }

        if let Some(c_bit) = self.external_commissioner_enabled {
            if c_bit {
                policy.set_external_commissioning_enabled(true);
            }
        }

        policy.set_commercial_commissioning_mode_disabled(true);
        policy.set_autonomous_enrollment_disabled(true);
        policy.set_network_key_provisioning_disabled(true);
        policy.set_to_ble_link_disabled(true);
        policy.set_non_ccm_routers_disabled(true);
        policy.set_reserved_bits(0x07);

        if let Some(rotation_time) = self.rotation_time_hours {
            policy.set_rotation_time(rotation_time);
        }

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
    }
}
