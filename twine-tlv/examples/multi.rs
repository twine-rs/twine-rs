use bytes::{Buf, BufMut};

use twine_rs_macros::Tlv;
use twine_tlv::{DecodeTlvValueUnchecked, TlvCollection, TlvLength, TryEncodeTlvValue};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(variants = [("Variant1", tlv_type = 0x03), ("Variant2", tlv_type = 0x04)], tlv_length = 3)]
struct ExampleData {
    inner: u8,
    bar: u16,
}

impl DecodeTlvValueUnchecked for ExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let inner = buffer.get_u8();
        let bar = buffer.get_u16();
        ExampleData { inner, bar }
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
#[tlv(tlv_type = 0x02, tlv_length = 4, derive_inner)]
struct MoreExampleData(u32);

fn main() {
    let data0 = ExampleData {
        inner: 0x2A,
        bar: 0xDEAD,
    };
    let data1 = MoreExampleData(0xDAFF_0D11);
    let mut collection = TlvCollection::<16>::default();

    collection.push(Variant1ExampleData::from(data0)).unwrap();
    collection.push(data1).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    // Example of transforming the data back to the inner type
    println!("Variant1ExampleData:\t{:02X?}", data0);
    let transform_data0: ExampleData = data0;
    println!("Transform ExampleData:\t{:02X?}", transform_data0);

    let data2 = Variant1ExampleData::from(ExampleData {
        inner: 0xFF,
        bar: 0xFFFF,
    });
    collection.replace(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<Variant1ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);

    let decoded = collection.decode_type_unchecked::<MoreExampleData>();
    println!("MoreExampleData:\t{:04x?}", decoded);
}
