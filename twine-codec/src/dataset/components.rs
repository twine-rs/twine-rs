bitflags::bitflags! {
    /// Represents the presence of different components in an Active or Pending Operational Dataset
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub(crate) struct Components: u16 {
        const ACTIVE = 1;
        const PENDING = 1 << 1;
        const NETWORK_KEY = 1 << 2;
        const NETWORK_NAME = 1 << 3;
        const EXTENDED_PAN_ID = 1 << 4;
        const MESH_LOCAL_PREFIX = 1 << 5;
        const DELAY = 1 << 6;
        const PAN_ID = 1 << 7;
        const CHANNEL = 1 << 8;
        const PSKC = 1 << 9;
        const SECURITY_POLICY = 1 << 10;
        const CHANNEL_MASK = 1 << 11;
    }
}

impl Components {
    pub(crate) fn active(&self) -> bool {
        self.contains(Self::ACTIVE)
    }

    pub(crate) fn pending(&self) -> bool {
        self.contains(Self::PENDING)
    }

    pub(crate) fn network_key(&self) -> bool {
        self.contains(Self::NETWORK_KEY)
    }

    pub(crate) fn network_name(&self) -> bool {
        self.contains(Self::NETWORK_NAME)
    }

    pub(crate) fn extended_pan_id(&self) -> bool {
        self.contains(Self::EXTENDED_PAN_ID)
    }

    pub(crate) fn mesh_local_prefix(&self) -> bool {
        self.contains(Self::MESH_LOCAL_PREFIX)
    }

    pub(crate) fn delay(&self) -> bool {
        self.contains(Self::DELAY)
    }

    pub(crate) fn pan_id(&self) -> bool {
        self.contains(Self::PAN_ID)
    }

    pub(crate) fn channel(&self) -> bool {
        self.contains(Self::CHANNEL)
    }

    pub(crate) fn pskc(&self) -> bool {
        self.contains(Self::PSKC)
    }

    pub(crate) fn security_policy(&self) -> bool {
        self.contains(Self::SECURITY_POLICY)
    }

    pub(crate) fn channel_mask(&self) -> bool {
        self.contains(Self::CHANNEL_MASK)
    }
}

impl From<u16> for Components {
    fn from(value: u16) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    static ALL_COMPONENTS: LazyLock<Components> = LazyLock::new(|| {
        Components::ACTIVE
            | Components::PENDING
            | Components::NETWORK_KEY
            | Components::NETWORK_NAME
            | Components::EXTENDED_PAN_ID
            | Components::MESH_LOCAL_PREFIX
            | Components::DELAY
            | Components::PAN_ID
            | Components::CHANNEL
            | Components::PSKC
            | Components::SECURITY_POLICY
            | Components::CHANNEL_MASK
    });

    /// Helper function to test only a single bitflag being set
    fn helper_contains_single_bitflag(bits: u16) {
        let components = Components::from(bits);

        if components == Components::ACTIVE {
            assert!(components.active());
            assert!(ALL_COMPONENTS.active());
        } else {
            assert!(!components.active());
        }

        if components == Components::PENDING {
            assert!(components.pending());
            assert!(ALL_COMPONENTS.pending());
        } else {
            assert!(!components.pending());
        }

        if components == Components::NETWORK_KEY {
            assert!(components.network_key());
            assert!(ALL_COMPONENTS.network_key());
        } else {
            assert!(!components.network_key());
        }

        if components == Components::NETWORK_NAME {
            assert!(components.network_name());
            assert!(ALL_COMPONENTS.network_name());
        } else {
            assert!(!components.network_name());
        }

        if components == Components::EXTENDED_PAN_ID {
            assert!(components.extended_pan_id());
            assert!(ALL_COMPONENTS.extended_pan_id());
        } else {
            assert!(!components.extended_pan_id());
        }

        if components == Components::MESH_LOCAL_PREFIX {
            assert!(components.mesh_local_prefix());
            assert!(ALL_COMPONENTS.mesh_local_prefix());
        } else {
            assert!(!components.mesh_local_prefix());
        }

        if components == Components::DELAY {
            assert!(components.delay());
            assert!(ALL_COMPONENTS.delay());
        } else {
            assert!(!components.delay());
        }

        if components == Components::PAN_ID {
            assert!(components.pan_id());
            assert!(ALL_COMPONENTS.pan_id());
        } else {
            assert!(!components.pan_id());
        }

        if components == Components::CHANNEL {
            assert!(components.channel());
            assert!(ALL_COMPONENTS.channel());
        } else {
            assert!(!components.channel());
        }

        if components == Components::PSKC {
            assert!(components.pskc());
            assert!(ALL_COMPONENTS.pskc());
        } else {
            assert!(!components.pskc());
        }

        if components == Components::SECURITY_POLICY {
            assert!(components.security_policy());
            assert!(ALL_COMPONENTS.security_policy());
        } else {
            assert!(!components.security_policy());
        }

        if components == Components::CHANNEL_MASK {
            assert!(components.channel_mask());
            assert!(ALL_COMPONENTS.channel_mask());
        } else {
            assert!(!components.channel_mask());
        }
    }

    #[test]
    fn contains_active() {
        helper_contains_single_bitflag(Components::ACTIVE.bits());

        let active = Components::from(Components::ACTIVE.bits());
        assert!(active.active());
    }

    #[test]
    fn contains_pending() {
        helper_contains_single_bitflag(Components::PENDING.bits());

        let pending = Components::from(Components::PENDING.bits());
        assert!(pending.pending());
    }

    #[test]
    fn contains_network_key() {
        helper_contains_single_bitflag(Components::NETWORK_KEY.bits());

        let network_key = Components::from(Components::NETWORK_KEY.bits());
        assert!(network_key.network_key());
    }

    #[test]
    fn contains_network_name() {
        helper_contains_single_bitflag(Components::NETWORK_NAME.bits());

        let network_name = Components::from(Components::NETWORK_NAME.bits());
        assert!(network_name.network_name());
    }

    #[test]
    fn contains_extended_pan_id() {
        helper_contains_single_bitflag(Components::EXTENDED_PAN_ID.bits());

        let extended_pan_id = Components::from(Components::EXTENDED_PAN_ID.bits());
        assert!(extended_pan_id.extended_pan_id());
    }

    #[test]
    fn contains_mesh_local_prefix() {
        helper_contains_single_bitflag(Components::MESH_LOCAL_PREFIX.bits());

        let mesh_local_prefix = Components::from(Components::MESH_LOCAL_PREFIX.bits());
        assert!(mesh_local_prefix.mesh_local_prefix());
    }

    #[test]
    fn contains_delay() {
        helper_contains_single_bitflag(Components::DELAY.bits());

        let delay = Components::from(Components::DELAY.bits());
        assert!(delay.delay());
    }

    #[test]
    fn contains_pan_id() {
        helper_contains_single_bitflag(Components::PAN_ID.bits());

        let pan_id = Components::from(Components::PAN_ID.bits());
        assert!(pan_id.pan_id());
    }

    #[test]
    fn contains_channel() {
        helper_contains_single_bitflag(Components::CHANNEL.bits());

        let channel = Components::from(Components::CHANNEL.bits());
        assert!(channel.channel());
    }

    #[test]
    fn contains_pskc() {
        helper_contains_single_bitflag(Components::PSKC.bits());

        let pskc = Components::from(Components::PSKC.bits());
        assert!(pskc.pskc());
    }

    #[test]
    fn contains_security_policy() {
        helper_contains_single_bitflag(Components::SECURITY_POLICY.bits());

        let security_policy = Components::from(Components::SECURITY_POLICY.bits());
        assert!(security_policy.security_policy());
    }

    #[test]
    fn contains_channel_mask() {
        helper_contains_single_bitflag(Components::CHANNEL_MASK.bits());

        let channel_mask = Components::from(Components::CHANNEL_MASK.bits());
        assert!(channel_mask.channel_mask());
    }
}
