use bytes::{Buf, BufMut};

use twine_macros::Tlv;
use twine_tlv::{
    DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, PutTlvLength, TlvCollection,
    TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue, TwineTlvError,
};

#[derive(Tlv)]
#[tlv(tlv_type = 0x03)]
pub(crate) struct ExampleData(pub(crate) Vec<u8>);

impl TlvLength for ExampleData {
    fn tlv_len(&self) -> usize {
        self.0.len()
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
    let mut collection = TlvCollection::<32>::new();

    collection.push(data0).unwrap();
    collection.push(data1).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    let data2 = ExampleData(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    collection.replace(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);
}
