#[cfg(any(test, feature = "alloc"))]
use alloc::vec::Vec;

#[cfg(any(test, feature = "alloc"))]
use alloc::string::String;

#[cfg(any(test, feature = "alloc"))]
use alloc::format;
use bytes::{BufMut, BytesMut};

use crate::{
    dataset,
    error::TwineCodecError,
    radio::{Channel, PanId},
};

mod components;
mod mesh_local_prefix;
mod network_key;
mod network_name;
mod pskc;
mod security_policy;
mod timestamp;
mod tlvs;
mod xpan;

pub use mesh_local_prefix::MeshLocalPrefix;
pub use network_key::NetworkKey;
pub use network_name::NetworkName;
pub use pskc::Pskc;
pub use security_policy::{SecurityPolicy, SecurityPolicyBuilder, VersionThreshold};
pub use timestamp::Timestamp;
pub use tlvs::MeshCopTlv;
pub use xpan::ExtendedPanId;

const OPERATIONAL_DATASET_MAX_LENGTH: usize = 254;

pub struct OperationalDatasetTlvsVec {
    tlvs: heapless::Vec<MeshCopTlv, 32>,
}

impl OperationalDatasetTlvsVec {
    fn type_name() -> &'static str {
        "OperationalDatasetTlvVec"
    }

    fn dataset_length(&self) -> usize {
        let mut length = 0;

        for tlv in self.tlvs.iter() {
            length += tlv.length();
        }

        length
    }

    fn is_valid(&self) -> bool {
        self.dataset_length() <= OPERATIONAL_DATASET_MAX_LENGTH
    }
}

impl TryFrom<OperationalDatasetTlvsVec> for OperationalDatasetTlvs {
    type Error = TwineCodecError;

    fn try_from(vec: OperationalDatasetTlvsVec) -> Result<Self, Self::Error> {
        let dataset_length = vec.dataset_length();
        if !dataset_length <= OPERATIONAL_DATASET_MAX_LENGTH {
            return Err(TwineCodecError::BufferMaxLength(
                Self::type_name(),
                OPERATIONAL_DATASET_MAX_LENGTH,
                dataset_length,
            ));
        }

        let mut buffer = BytesMut::with_capacity(OPERATIONAL_DATASET_MAX_LENGTH);
        for tlv in vec.tlvs.iter() {
            tlv.encode(&mut buffer);
        }

        let array = &buffer.freeze()[..]
            .try_into()
            .map_err(|_| TwineCodecError::BufferBytesConversion(Self::type_name()))?;

        Ok(Self {
            tlvs: *array,
            length: dataset_length as u8,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OperationalDatasetTlvs {
    tlvs: [u8; OPERATIONAL_DATASET_MAX_LENGTH],
    length: u8,
}

impl OperationalDatasetTlvs {
    fn type_name() -> &'static str {
        "OperationalDatasetTlvs"
    }

    #[cfg(any(test, feature = "alloc"))]
    pub fn as_string(&self) -> String {
        let mut string = String::new();

        for byte in self.tlvs {
            string = format!("{string}{byte:02x}");
        }

        string
    }
}

impl Default for OperationalDatasetTlvs {
    fn default() -> Self {
        Self {
            tlvs: [0; OPERATIONAL_DATASET_MAX_LENGTH],
            length: 0,
        }
    }
}

#[cfg(any(test, feature = "alloc"))]
impl TryFrom<Vec<u8>> for OperationalDatasetTlvs {
    type Error = TwineCodecError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let length = bytes.len();

        if length > OPERATIONAL_DATASET_MAX_LENGTH {
            return Err(TwineCodecError::BufferMaxLength(
                Self::type_name(),
                OPERATIONAL_DATASET_MAX_LENGTH,
                length,
            ));
        }

        let mut tlvs = [0_u8; OPERATIONAL_DATASET_MAX_LENGTH];
        bytes.iter().enumerate().for_each(|(i, byte)| {
            tlvs[i] = *byte;
        });

        Ok(Self {
            tlvs,
            length: length as u8,
        })
    }
}

pub struct OperationalDataset {
    active: Option<Timestamp>,
    pending: Option<Timestamp>,
    key: NetworkKey,
    name: NetworkName,
    extended_pan_id: ExtendedPanId,
    mesh_local_prefix: MeshLocalPrefix,
    delay: u32,
    pan_id: PanId,
    channel: Channel,
    pskc: Pskc,
    security_policy: SecurityPolicy,
}

impl OperationalDataset {
    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    pub fn is_pending(&self) -> bool {
        self.pending.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    extern crate alloc;

    #[test]
    fn try_from_vec_u8_for_operational_dataset_tlvs() {
        let expected = OperationalDatasetTlvs {
            tlvs: [0x27; OPERATIONAL_DATASET_MAX_LENGTH],
            length: OPERATIONAL_DATASET_MAX_LENGTH as u8,
        };

        // Input is max length
        let input_max_length = vec![0x27; OPERATIONAL_DATASET_MAX_LENGTH];
        let test = OperationalDatasetTlvs::try_from(input_max_length)
            .expect("Could not convert array to OperationalDatasetTlvs");
        assert_eq!(expected, test);

        // Input is too long
        let input_too_long = vec![0x27; OPERATIONAL_DATASET_MAX_LENGTH + 1];
        let test = OperationalDatasetTlvs::try_from(input_too_long);
        let expected_err = TwineCodecError::BufferMaxLength(
            OperationalDatasetTlvs::type_name(),
            OPERATIONAL_DATASET_MAX_LENGTH,
            OPERATIONAL_DATASET_MAX_LENGTH + 1,
        );
        assert_eq!(Err(expected_err), test);

        // Input is standard length
        let mut expected = OperationalDatasetTlvs {
            tlvs: [0x00; OPERATIONAL_DATASET_MAX_LENGTH],
            length: 4 as u8,
        };
        expected.tlvs[0] = 0x27;
        expected.tlvs[1] = 0x27;
        expected.tlvs[2] = 0x27;
        expected.tlvs[3] = 0x27;

        let mut input = vec![0xFF; 4];
        input[0] = 0x27;
        input[1] = 0x27;
        input[2] = 0x27;
        input[3] = 0x27;

        let test = OperationalDatasetTlvs::try_from(input)
            .expect("Could not convert array to OperationalDatasetTlvs");
        assert_eq!(expected, test);

        // Input is empty
        let expected = OperationalDatasetTlvs {
            tlvs: [0x00; OPERATIONAL_DATASET_MAX_LENGTH],
            length: 0 as u8,
        };

        let empty = Vec::new();
        let test = OperationalDatasetTlvs::try_from(empty)
            .expect("Could not convert array to OperationalDatasetTlvs");
        assert_eq!(expected, test);
    }
}
