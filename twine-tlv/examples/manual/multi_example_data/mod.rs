//! Example of manually implementing TLV traits for a type that is:
//! - Multiple variants (a single type that can be represented as multiple TLV types)
//! - Multiple fields
//! - A constant length

use bytes::{Buf, BufMut};

use twine_tlv::prelude::*;

mod variant0;
mod variant1;

pub use variant0::Variant0MultiExampleData;
pub use variant1::Variant1MultiExampleData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MultiExampleData {
    inner: u8,
    other: u16,
}

impl MultiExampleData {
    pub fn new(inner: u8, other: u16) -> Self {
        Self { inner, other }
    }
}

impl TlvType for MultiExampleData {
    const TLV_TYPE: u8 = 0x01;
}

impl TlvConstantMetadata for MultiExampleData {
    const TLV_LEN: usize = 3; // 1 byte for inner + 2 bytes for other
}

impl TlvLength for MultiExampleData {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for MultiExampleData {}

impl DecodeTlvUnchecked for MultiExampleData {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for MultiExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let inner = buffer.get_u8();
        let other = buffer.get_u16();
        MultiExampleData { inner, other }
    }
}

impl TryEncodeTlvValue for MultiExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u8(self.inner);
        buffer.put_u16(self.other);
        Ok(self.tlv_len())
    }
}
