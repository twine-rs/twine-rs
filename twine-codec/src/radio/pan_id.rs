/// IEEE 802.15.4 PAN ID
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PanId(u16);

impl PanId {
    /// Create a new IEEE 802.15.4 PAN ID
    pub fn new(pan_id: u16) -> Self {
        Self(pan_id)
    }

    /// Create a new IEEE 802.15.4 Broadcast PAN ID
    pub fn broadcast() -> Self {
        Self(0xffff)
    }
}

impl From<PanId> for u16 {
    fn from(value: PanId) -> Self {
        value.0
    }
}

impl From<u16> for PanId {
    fn from(pan_id: u16) -> Self {
        Self(pan_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadcast() {
        assert_eq!(PanId::broadcast(), PanId(0xffff));
    }
}
