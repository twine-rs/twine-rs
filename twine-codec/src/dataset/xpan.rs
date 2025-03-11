pub(crate) const EXT_PAN_ID_SIZE: usize = 8;

/// IEEE 802.15.4 Extended PAN ID
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPanId([u8; EXT_PAN_ID_SIZE]);

impl From<ExtendedPanId> for u64 {
    fn from(value: ExtendedPanId) -> Self {
        u64::from_be_bytes(value.0)
    }
}

impl From<u64> for ExtendedPanId {
    fn from(id: u64) -> Self {
        Self(id.to_be_bytes())
    }
}
