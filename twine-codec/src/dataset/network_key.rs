#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

use core::num::ParseIntError;
use core::str::FromStr;

const NETWORK_KEY_SIZE: usize = 16;

/// A Thread Network Key
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NetworkKey([u8; NETWORK_KEY_SIZE]);

#[cfg(any(test, feature = "alloc"))]
impl From<NetworkKey> for Vec<u8> {
    fn from(value: NetworkKey) -> Self {
        value.0.to_vec()
    }
}

impl From<u128> for NetworkKey {
    fn from(key: u128) -> Self {
        Self(key.to_be_bytes())
    }
}

impl AsRef<[u8]> for NetworkKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for NetworkKey {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl FromStr for NetworkKey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = u128::from_str_radix(s, 16)?;
        Ok(Self::from(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::std::borrow::ToOwned;
    extern crate alloc;

    const EXPECTED_KEY_STR: &str = "0123456789abcdef0123456789abcdef";
    const EXPECTED_KEY_U128: u128 = 0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;
    const EXPECTED_KEY_BYTES: [u8; NETWORK_KEY_SIZE] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef,
    ];

    #[test]
    fn success_from_str() {
        let key =
            NetworkKey::from_str(EXPECTED_KEY_STR).expect("Failed to parse network key string");
        assert_eq!(key.0, EXPECTED_KEY_BYTES);
    }

    #[test]
    fn fail_from_str() {
        let key = NetworkKey::from_str("not a valid network key string");
        assert!(key.is_err());

        let too_long = EXPECTED_KEY_STR.to_owned() + "0927";
        let key = NetworkKey::from_str(&too_long);
        assert!(key.is_err());
    }

    #[test]
    fn success_from_u128() {
        let key = NetworkKey::from(EXPECTED_KEY_U128);
        assert_eq!(key.0, EXPECTED_KEY_BYTES);
    }

    #[test]
    fn success_as_ref() {
        let key = NetworkKey::from(EXPECTED_KEY_U128);
        let bytes = key.as_ref();
        assert_eq!(bytes, &EXPECTED_KEY_BYTES);
        assert_eq!(bytes.len(), NETWORK_KEY_SIZE);
    }

    #[test]
    fn success_as_mut() {
        let mut key = NetworkKey::from(EXPECTED_KEY_U128);
        let bytes = key.as_mut();
        assert_eq!(bytes, &EXPECTED_KEY_BYTES);
        assert_eq!(bytes.len(), NETWORK_KEY_SIZE);
    }
}
