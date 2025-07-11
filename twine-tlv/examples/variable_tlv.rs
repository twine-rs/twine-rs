use bytes::{Buf, BufMut};

use twine_macros::Tlv;
use twine_tlv::{
    write_tlv, DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, TlvCollection, TlvLength,
    TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue, TwineTlvError,
};

#[derive(Clone)]
pub(crate) struct ExampleData(pub(crate) Vec<u8>);

impl TlvType for ExampleData {
    const TLV_TYPE: u8 = 0x03;
}

impl TlvLength for ExampleData {
    fn tlv_len(&self) -> usize {
        self.0.len()
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
        let mut data = Vec::new();

        while buffer.has_remaining() {
            data.push(buffer.get_u8());
        }

        ExampleData(data)
    }
}

impl TryEncodeTlv for ExampleData {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, &self.clone())?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for ExampleData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();

        if buffer.len() < self.tlv_len() {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        for &byte in &self.0 {
            buffer.put_u8(byte);
        }

        Ok(self.tlv_len())
    }
}

#[derive(Tlv)]
#[tlv(tlv_type = 0x04)]
pub(crate) struct ExampleStringData(pub(crate) String);

impl TlvLength for ExampleStringData {
    fn tlv_len(&self) -> usize {
        self.0.len()
    }
}

impl DecodeTlvValueUnchecked for ExampleStringData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let buffer = buffer.as_ref();
        let string = String::from_utf8_lossy(buffer).to_string();
        ExampleStringData(string)
    }
}

impl TryEncodeTlvValue for ExampleStringData {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();

        if buffer.len() < self.tlv_len() {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        for &byte in self.0.as_bytes() {
            buffer.put_u8(byte);
        }

        Ok(self.tlv_len())
    }
}

fn main() {
    let data0 = ExampleData(vec![0xDE, 0xAD, 0xC0, 0xDE]);
    let data1 = ExampleStringData("Variable TLV Example".to_string());
    let mut collection = TlvCollection::<32>::default();

    collection.push(data0).unwrap();
    collection.push(data1).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    let data2 = ExampleData(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    collection.replace(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);
}
