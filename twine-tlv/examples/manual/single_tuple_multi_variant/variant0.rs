use bytes::Buf;

use twine_tlv::prelude::*;

use crate::single_tuple_multi_variant::SingleTupleMultiVariant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Variant0SingleTupleMultiVariant(SingleTupleMultiVariant);

impl Variant0SingleTupleMultiVariant {
    pub fn _new(value: u32) -> Self {
        Self(SingleTupleMultiVariant(value))
    }
}

impl TlvType for Variant0SingleTupleMultiVariant {
    const TLV_TYPE: u8 = 0x05;
}

impl TlvConstantMetadata for Variant0SingleTupleMultiVariant {
    const TLV_LEN: usize = 4; // 4 bytes for u32
}

impl TlvLength for Variant0SingleTupleMultiVariant {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for Variant0SingleTupleMultiVariant {}

impl From<Variant0SingleTupleMultiVariant> for SingleTupleMultiVariant {
    fn from(value: Variant0SingleTupleMultiVariant) -> Self {
        value.0
    }
}

impl From<SingleTupleMultiVariant> for Variant0SingleTupleMultiVariant {
    fn from(value: SingleTupleMultiVariant) -> Self {
        Variant0SingleTupleMultiVariant(value)
    }
}

impl core::ops::Deref for Variant0SingleTupleMultiVariant {
    type Target = SingleTupleMultiVariant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DecodeTlvUnchecked for Variant0SingleTupleMultiVariant {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for Variant0SingleTupleMultiVariant {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let inner: SingleTupleMultiVariant =
            ::twine_tlv::DecodeTlvValueUnchecked::decode_tlv_value_unchecked(buffer);
        Variant0SingleTupleMultiVariant(inner)
    }
}

impl TryEncodeTlv for Variant0SingleTupleMultiVariant {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        write_tlv(buffer, Self::TLV_TYPE, self)
    }
}

impl TryEncodeTlvValue for Variant0SingleTupleMultiVariant {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0 .0.try_encode_tlv_value(buffer)
    }
}
