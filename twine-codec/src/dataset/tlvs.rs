use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_enum::IntoPrimitive;

use super::{
    security_policy::SecurityPolicy, ExtendedPanId, NetworkKey, NetworkName, Pskc, Timestamp,
};
use crate::{
    commissioner::SteeringData,
    error::TwineCodecError,
    radio::{Channel, PanId},
};

#[derive(Clone, Debug)]
pub struct MeshCopTlv {
    tag: MeshCopTlvTag,
    length: u8,
    value: Bytes,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum MeshCopTlvTag {
    Channel = 0,
    PanId = 1,
    ExtendedPanId = 2,
    NetworkName = 3,
    Pskc = 4,
    NetworkKey = 5,
    NetworkMeshLocalPrefix = 7,
    SteeringData = 8,
    SecurityPolicy = 12,
    ActiveTimestamp = 14,
    ChannelMask = 53,
    WakeUpChannel = 74,
    Unknown(u8),
}

impl From<u8> for MeshCopTlvTag {
    fn from(value: u8) -> Self {
        match value {
            0 => MeshCopTlvTag::Channel,
            1 => MeshCopTlvTag::PanId,
            2 => MeshCopTlvTag::ExtendedPanId,
            3 => MeshCopTlvTag::NetworkName,
            4 => MeshCopTlvTag::Pskc,
            5 => MeshCopTlvTag::NetworkKey,
            7 => MeshCopTlvTag::NetworkMeshLocalPrefix,
            8 => MeshCopTlvTag::SteeringData,
            12 => MeshCopTlvTag::SecurityPolicy,
            14 => MeshCopTlvTag::ActiveTimestamp,
            74 => MeshCopTlvTag::WakeUpChannel,
            53 => MeshCopTlvTag::ChannelMask,
            _ => MeshCopTlvTag::Unknown(value),
        }
    }
}

impl From<MeshCopTlvTag> for u8 {
    fn from(value: MeshCopTlvTag) -> Self {
        match value {
            MeshCopTlvTag::Channel => 0,
            MeshCopTlvTag::PanId => 1,
            MeshCopTlvTag::ExtendedPanId => 2,
            MeshCopTlvTag::NetworkName => 3,
            MeshCopTlvTag::Pskc => 4,
            MeshCopTlvTag::NetworkKey => 5,
            MeshCopTlvTag::NetworkMeshLocalPrefix => 7,
            MeshCopTlvTag::SteeringData => 8,
            MeshCopTlvTag::SecurityPolicy => 12,
            MeshCopTlvTag::ActiveTimestamp => 14,
            MeshCopTlvTag::WakeUpChannel => 74,
            MeshCopTlvTag::ChannelMask => 53,
            MeshCopTlvTag::Unknown(value) => value,
        }
    }
}

impl MeshCopTlv {
    fn type_name() -> &'static str {
        "MeshCopTlv"
    }

    fn new(tag: MeshCopTlvTag, length: u8, value: Bytes) -> Self {
        Self { tag, length, value }
    }

    fn tag(&self) -> MeshCopTlvTag {
        self.tag
    }

    pub(crate) fn length(&self) -> usize {
        self.length as usize
    }

    fn value(&self) -> &Bytes {
        &self.value
    }

    /// Attempt to decode a TLV from a buffer of bytes and advance the buffer.
    pub fn decode(bytes: &mut Bytes) -> Result<Self, TwineCodecError> {
        let tag = bytes.get_u8().into();
        let length = bytes.get_u8();
        let value = bytes.split_to(length as usize);

        Ok(MeshCopTlv::new(tag, length, value))
    }

    pub fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u8(u8::from(self.tag()));
        bytes.put_u8(self.length);
        bytes.put_slice(&self.value);
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    const CHANNEL_TLV_BYTES: [u8; 5] = [0x00, 0x03, 0x00, 0x00, 0x16];
    const PAN_ID_TLV_BYTES: [u8; 4] = [0x01, 0x02, 0xe0, 0x19];
    const EXTENDED_PAN_ID_TLV_BYTES: [u8; 10] =
        [0x02, 0x08, 0x93, 0x33, 0xcb, 0x0d, 0x9c, 0x0e, 0xec, 0x48];
    const NETWORK_NAME_TLV_BYTES: [u8; 17] = [
        0x03, 0x0f, 0x4f, 0x70, 0x65, 0x6e, 0x54, 0x68, 0x72, 0x65, 0x61, 0x64, 0x2d, 0x65, 0x30,
        0x31, 0x39,
    ];
    const PSKC_TLV_BYTES: [u8; 18] = [
        0x04, 0x10, 0xa3, 0x3e, 0x84, 0xe9, 0xd7, 0xed, 0x60, 0x2e, 0x21, 0x3d, 0x39, 0x22, 0xc7,
        0x30, 0x9d, 0x57,
    ];
    const NETWORK_KEY_TLV_BYTES: [u8; 18] = [
        0x05, 0x10, 0x90, 0x21, 0x2c, 0xda, 0x44, 0x73, 0x4b, 0xca, 0xa7, 0x68, 0x6d, 0xa5, 0xdb,
        0x31, 0xa0, 0x55,
    ];
    const NETWORK_MESH_LOCAL_PREFIX_TLV_BYTES: [u8; 10] =
        [0x07, 0x08, 0xfd, 0x76, 0x14, 0x46, 0x3a, 0x6f, 0x7b, 0xc1];
    const SECURITY_POLICY_TLV_BYTES: [u8; 6] = [0x0c, 0x04, 0x02, 0xa0, 0xf7, 0xf8];
    const ACTIVE_TIMESTAMP_TLV_BYTES: [u8; 10] =
        [0x0e, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00];
    const CHANNEL_MASK_TLV_BYTES: [u8; 8] = [0x35, 0x06, 0x00, 0x04, 0x00, 0x1f, 0xff, 0xe0];
    const WAKEUP_CHANNEL_TLV_BYTES: [u8; 5] = [0x4a, 0x03, 0x00, 0x00, 0x1a];

    fn helper_test_decode(bytes: &'static [u8], expected_tag: MeshCopTlvTag) -> MeshCopTlv {
        let mut bytes = Bytes::from_static(bytes);
        let tlv = MeshCopTlv::decode(&mut bytes).expect("Could not decode TLV");
        println!("tlv: {tlv:?}");

        assert_eq!(tlv.tag(), expected_tag);
        assert_eq!(u8::from(tlv.tag()), u8::from(expected_tag));

        tlv
    }

    #[test]
    fn tlv_0_channel() {
        let _ = helper_test_decode(&CHANNEL_TLV_BYTES, MeshCopTlvTag::Channel);
    }

    #[test]
    fn tlv_1_pan_id() {
        let _ = helper_test_decode(&PAN_ID_TLV_BYTES, MeshCopTlvTag::PanId);
    }

    #[test]
    fn tlv_2_extended_pan_id() {
        let _ = helper_test_decode(&EXTENDED_PAN_ID_TLV_BYTES, MeshCopTlvTag::ExtendedPanId);
    }

    #[test]
    fn tlv_3_network_name() {
        let _ = helper_test_decode(&NETWORK_NAME_TLV_BYTES, MeshCopTlvTag::NetworkName);
    }

    #[test]
    fn tlv_4_pskc() {
        let _ = helper_test_decode(&PSKC_TLV_BYTES, MeshCopTlvTag::Pskc);
    }

    #[test]
    fn tlv_5_network_key() {
        let _ = helper_test_decode(&NETWORK_KEY_TLV_BYTES, MeshCopTlvTag::NetworkKey);
    }

    #[test]
    fn tlv_7_network_mesh_local_prefix() {
        let _ = helper_test_decode(
            &NETWORK_MESH_LOCAL_PREFIX_TLV_BYTES,
            MeshCopTlvTag::NetworkMeshLocalPrefix,
        );
    }

    #[test]
    fn tlv_12_security_policy() {
        let _ = helper_test_decode(&SECURITY_POLICY_TLV_BYTES, MeshCopTlvTag::SecurityPolicy);
    }

    #[test]
    fn tlv_14_active_timestamp() {
        let _ = helper_test_decode(&ACTIVE_TIMESTAMP_TLV_BYTES, MeshCopTlvTag::ActiveTimestamp);
    }

    #[test]
    fn tlv_53_channel_mask() {
        let _ = helper_test_decode(&CHANNEL_MASK_TLV_BYTES, MeshCopTlvTag::ChannelMask);
    }

    #[test]
    fn tlv_74_wake_up_channel() {
        let _ = helper_test_decode(&WAKEUP_CHANNEL_TLV_BYTES, MeshCopTlvTag::WakeUpChannel);
    }
}
