use bytes::{Buf, BufMut};

use twine_macros::Tlv;
use twine_tlv::{
    write_tlv, DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, TlvCollection,
    TlvConstantMetadata, TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue,
    TwineTlvError,
};

#[derive(Tlv, Clone, Copy, Debug, Eq, PartialEq)]
#[tlv(variant = "Variant1", tlv_type = 0x03, tlv_length = 3)]
#[tlv(variant = "Variant2", tlv_type = 0x21, tlv_length = 3)]
struct ExampleData {
    foo: u8,
    bar: u16,
}

impl DecodeTlvValueUnchecked for ExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let foo = buffer.get_u8();
        let bar = buffer.get_u16();
        ExampleData { foo, bar }
    }
}

impl TryEncodeTlvValue for ExampleData {
    fn try_encode_tlv_value(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, twine_tlv::error::TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u8(self.foo);
        buffer.put_u16(self.bar);
        Ok(self.tlv_len())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x02, tlv_length = 4)]
struct MoreExampleData {
    baz: u32,
}

impl DecodeTlvValueUnchecked for MoreExampleData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let baz = buffer.get_u32();
        MoreExampleData { baz }
    }
}

impl TryEncodeTlvValue for MoreExampleData {
    fn try_encode_tlv_value(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, twine_tlv::error::TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u32(self.baz);
        Ok(self.tlv_len())
    }
}

fn main() {
    let data0 = ExampleData {
        foo: 0x2A,
        bar: 0xDEAD,
    };
    let data1 = MoreExampleData { baz: 0xDAFF_0D11 };
    let mut collection = TlvCollection::<16>::default();

    collection.push(Variant2ExampleData(data0)).unwrap();
    collection.push(data1).unwrap();
    println!("Push data:\t\t{:02X?}", collection);

    let data2 = Variant2ExampleData(ExampleData {
        foo: 0xFF,
        bar: 0xFFFF,
    });
    collection.replace(data2).unwrap();
    println!("Replace ExampleData:\t{:02X?}", collection);

    collection.remove::<Variant2ExampleData>();
    println!("Remove ExampleData:\t{:02X?}", collection);
}
