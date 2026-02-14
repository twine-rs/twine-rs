use bytes::Buf;

use twine_tlv::prelude::*;

use crate::single_tuple_multi_variant::SingleTupleMultiVariant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Variant1SingleTupleMultiVariant(SingleTupleMultiVariant);

impl Variant1SingleTupleMultiVariant {
    pub fn _new(value: u32) -> Self {
        Self(SingleTupleMultiVariant(value))
    }
}

impl TlvType for Variant1SingleTupleMultiVariant {
    const TLV_TYPE: u8 = 0x55;
}

impl TlvConstantMetadata for Variant1SingleTupleMultiVariant {
    const TLV_LEN: usize = 4; // 4 bytes for u32
}

impl TlvLength for Variant1SingleTupleMultiVariant {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl TlvMetadata for Variant1SingleTupleMultiVariant {}

impl From<Variant1SingleTupleMultiVariant> for SingleTupleMultiVariant {
    fn from(value: Variant1SingleTupleMultiVariant) -> Self {
        value.0
    }
}

impl From<SingleTupleMultiVariant> for Variant1SingleTupleMultiVariant {
    fn from(value: SingleTupleMultiVariant) -> Self {
        Variant1SingleTupleMultiVariant(value)
    }
}

impl core::ops::Deref for Variant1SingleTupleMultiVariant {
    type Target = SingleTupleMultiVariant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DecodeTlvUnchecked for Variant1SingleTupleMultiVariant {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for Variant1SingleTupleMultiVariant {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let inner: SingleTupleMultiVariant =
            ::twine_tlv::DecodeTlvValueUnchecked::decode_tlv_value_unchecked(buffer);
        Variant1SingleTupleMultiVariant(inner)
    }
}

impl TryEncodeTlv for Variant1SingleTupleMultiVariant {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        write_tlv(buffer, Self::TLV_TYPE, self)
    }
}

impl TryEncodeTlvValue for Variant1SingleTupleMultiVariant {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0 .0.try_encode_tlv_value(buffer)
    }
}
