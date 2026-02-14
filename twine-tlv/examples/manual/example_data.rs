//! Example of manually implementing TLV traits for a type that is:
//! - A single variant (only represents a single TLV type)
//! - Multiple fields
//! - A constant length

use bytes::{Buf, BufMut};

use twine_tlv::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExampleData {
    inner: u8,
    other: u16,
}

impl ExampleData {
    pub fn new(inner: u8, other: u16) -> Self {
        Self { inner, other }
    }
}

impl TlvType for ExampleData {
    const TLV_TYPE: u8 = 0x02;
}

impl TlvConstantMetadata for ExampleData {
    const TLV_LEN: usize = 3; // 1 byte for inner + 2 bytes for other
}

impl TlvLength for ExampleData {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for ExampleData {}

impl DecodeTlvUnchecked for ExampleData {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for ExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let inner = buffer.get_u8();
        let other = buffer.get_u16();
        ExampleData { inner, other }
    }
}

impl TryEncodeTlv for ExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, self)?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for ExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u8(self.inner);
        buffer.put_u16(self.other);
        Ok(self.tlv_len())
    }
}
