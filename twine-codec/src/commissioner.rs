#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

use crate::error::TwineCodecError;

const JOINER_PSKD_MAX_LEN: usize = 32;
const PROVISIONING_URL_MAX_LEN: usize = 64;
const THREAD_DOMAIN_NAME_MAX_LEN: usize = 16;
const VENDOR_NAME_MAX_LEN: usize = 32;
const VENDOR_MODEL_MAX_LEN: usize = 32;
const VENDOR_SW_VERSION_MAX_LEN: usize = 16;
const VENDOR_DATA_MAX_LEN: usize = 64;

const STEERING_DATA_MAX_LEN: usize = 16;

pub struct JoinerPskd {
    pskd: [u8; JOINER_PSKD_MAX_LEN + 1],
}

// pub struct ProvisioningUrl(String);

// pub struct ThreadVendorInfo {
//     name: String,
//     model: String,
//     sw_version: String,
//     data: String,
// }

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SteeringData {
    length: u8,
    bytes: [u8; STEERING_DATA_MAX_LEN],
}

impl SteeringData {
    fn type_name() -> &'static str {
        "SteeringData"
    }
}

#[cfg(any(test, feature = "alloc"))]
impl TryFrom<Vec<u8>> for SteeringData {
    type Error = TwineCodecError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let length = bytes.len();

        if length > STEERING_DATA_MAX_LEN {
            return Err(TwineCodecError::BufferMaxLength(
                Self::type_name(),
                STEERING_DATA_MAX_LEN,
                length,
            ));
        }

        let mut b = [0_u8; STEERING_DATA_MAX_LEN];
        bytes.iter().enumerate().for_each(|(i, byte)| {
            b[i] = *byte;
        });

        Ok(Self {
            length: length as u8,
            bytes: b,
        })
    }
}
