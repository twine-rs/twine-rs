use twine_tlv::prelude::*;

use super::MultiExampleData;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Variant1MultiExampleData(MultiExampleData);

impl TlvMetadata for Variant1MultiExampleData {}

/// The first variant of `MultiExampleData`
///
/// This is a newtype wrapper around `MultiExampleData` that implements
/// a new TLV type and but same length as `MultiExampleData`.
impl TlvType for Variant1MultiExampleData {
    const TLV_TYPE: u8 = 0x51;
}

impl TlvConstantMetadata for Variant1MultiExampleData {
    const TLV_LEN: usize = 3; // 1 byte for inner + 2 bytes for other
}

impl TlvLength for Variant1MultiExampleData {
    fn tlv_len(&self) -> usize {
        Self::TLV_LEN
    }

    fn tlv_len_is_constant() -> bool {
        true
    }
}

impl From<Variant1MultiExampleData> for MultiExampleData {
    fn from(value: Variant1MultiExampleData) -> Self {
        value.0
    }
}

impl From<MultiExampleData> for Variant1MultiExampleData {
    fn from(value: MultiExampleData) -> Self {
        Variant1MultiExampleData(value)
    }
}

impl core::ops::Deref for Variant1MultiExampleData {
    type Target = MultiExampleData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryEncodeTlv for Variant1MultiExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        write_tlv(buffer, Self::TLV_TYPE, self)
    }
}

impl TryEncodeTlvValue for Variant1MultiExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        self.0.try_encode_tlv_value(buffer)
    }
}

impl DecodeTlvUnchecked for Variant1MultiExampleData {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for Variant1MultiExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        Variant1MultiExampleData(MultiExampleData::decode_tlv_value_unchecked(buffer))
    }
}
