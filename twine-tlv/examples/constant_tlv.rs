use bytes::{Buf, BufMut};

use twine_macros::Tlv;
use twine_tlv::{
    write_tlv, DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, TlvCollection,
    TlvConstantMetadata, TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue,
    TwineTlvError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleData {
    inner: u8,
    bar: u16,
}

impl TlvType for ExampleData {
    const TLV_TYPE: u8 = 0x01;
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

impl TlvConstantMetadata for ExampleData {
    const TLV_LEN: usize = 3; // 1 byte for inner + 2 bytes for bar
}

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
        let bar = buffer.get_u16();
        ExampleData { inner, bar }
    }
}

impl TryEncodeTlv for ExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, self)?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for ExampleData {
    fn try_encode_tlv_value(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, twine_tlv::error::TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u8(self.inner);
        buffer.put_u16(self.bar);
        Ok(self.tlv_len())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x02, tlv_length = 4)]
struct MoreExampleData {
    inner: u32,
}

impl DecodeTlvValueUnchecked for MoreExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let inner = buffer.get_u32();
        MoreExampleData { inner }
    }
}

impl TryEncodeTlvValue for MoreExampleData {
    fn try_encode_tlv_value(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, twine_tlv::error::TwineTlvError> {
        let mut buffer = buffer;
        buffer.put_u32(self.inner);
        Ok(self.tlv_len())
    }
}

fn main() {
    let data0 = ExampleData {
        inner: 0x2A,
        bar: 0xDEAD,
    };
    let data1 = MoreExampleData { inner: 0xDAFF_0D11 };
    let mut collection = TlvCollection::<16>::default();

    collection.push(data0).unwrap();
    collection.push(data1).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    let data2 = ExampleData {
        inner: 0xFF,
        bar: 0xFFFF,
    };
    collection.replace(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);
}
