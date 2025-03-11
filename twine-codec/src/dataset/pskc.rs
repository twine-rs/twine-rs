#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

const PSKC_MAX_SIZE: usize = 16;

/// A Thread PSKc
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pskc([u8; PSKC_MAX_SIZE]);

#[cfg(any(test, feature = "alloc"))]
impl From<Pskc> for Vec<u8> {
    fn from(value: Pskc) -> Self {
        value.0.to_vec()
    }
}

impl From<Pskc> for u128 {
    fn from(value: Pskc) -> Self {
        u128::from_be_bytes(value.0)
    }
}

impl From<u128> for Pskc {
    fn from(pskc: u128) -> Self {
        Self(pskc.to_be_bytes())
    }
}
