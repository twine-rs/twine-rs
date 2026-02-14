//! Example of manually implementing TLV traits for a type that is:
//! - A multiple variants (a single type that can be represented as multiple TLV types)
//! - A single field part of a tuple struct
//! - A constant length

use bytes::Buf;

use twine_tlv::prelude::*;

mod variant0;
mod variant1;

pub use variant0::Variant0SingleTupleMultiVariant;
pub use variant1::Variant1SingleTupleMultiVariant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SingleTupleMultiVariant(u32);

impl SingleTupleMultiVariant {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl TlvType for SingleTupleMultiVariant {
    const TLV_TYPE: u8 = 0x05;
}

impl TlvConstantMetadata for SingleTupleMultiVariant {
    const TLV_LEN: usize = 4; // 4 bytes for u32
}

impl TlvLength for SingleTupleMultiVariant {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for SingleTupleMultiVariant {}

impl DecodeTlvUnchecked for SingleTupleMultiVariant {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for SingleTupleMultiVariant {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let value = buffer.get_u32();
        SingleTupleMultiVariant(value)
    }
}

impl TryEncodeTlv for SingleTupleMultiVariant {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        write_tlv(buffer, Self::TLV_TYPE, self)
    }
}

impl TryEncodeTlvValue for SingleTupleMultiVariant {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0.try_encode_tlv_value(buffer)
    }
}
