//! Example of manually implementing TLV traits for a type that is:
//! - A single variant (only represents a single TLV type)
//! - A single field
//! - A constant length

use bytes::{Buf, BufMut};

use twine_tlv::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MoreExampleData {
    inner: u32,
}

impl MoreExampleData {
    pub fn new(inner: u32) -> Self {
        Self { inner }
    }
}

impl TlvType for MoreExampleData {
    const TLV_TYPE: u8 = 0x03;
}

impl TlvConstantMetadata for MoreExampleData {
    const TLV_LEN: usize = 4; // 4 byte for inner
}

impl TlvLength for MoreExampleData {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for MoreExampleData {}

impl DecodeTlvUnchecked for MoreExampleData {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for MoreExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let inner = buffer.get_u32();
        MoreExampleData { inner }
    }
}

impl TryEncodeTlv for MoreExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, self)?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for MoreExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u32(self.inner);
        Ok(self.tlv_len())
    }
}
