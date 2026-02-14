//! Example of manually implementing TLV traits for a type that is:
//! - A single variant (only represents a single TLV type)
//! - A single field part of a tuple struct
//! - A constant length

use bytes::Buf;

use twine_tlv::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SingleTupleExampleData(u32);

impl SingleTupleExampleData {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl TlvType for SingleTupleExampleData {
    const TLV_TYPE: u8 = 0x04;
}

impl TlvConstantMetadata for SingleTupleExampleData {
    const TLV_LEN: usize = 4; // 4 bytes for u32
}

impl TlvLength for SingleTupleExampleData {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for SingleTupleExampleData {}

impl DecodeTlvUnchecked for SingleTupleExampleData {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for SingleTupleExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let value = buffer.get_u32();
        SingleTupleExampleData(value)
    }
}

impl TryEncodeTlv for SingleTupleExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, self)?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for SingleTupleExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0.try_encode_tlv_value(buffer)
    }
}
