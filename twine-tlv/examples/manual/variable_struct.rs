//! Example of manually implementing TLV traits for a type that is:
//! - A single variant (only represents a single TLV type)
//! - Multiple fields
//! - Variable length

use bytes::{Buf, BufMut};

use twine_tlv::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableStruct {
    pub inner0: Vec<u8>,
    pub inner1: Vec<u16>,
}

impl VariableStruct {
    pub fn _new(inner0: Vec<u8>, inner1: Vec<u16>) -> Self {
        Self { inner0, inner1 }
    }
}

impl TlvType for VariableStruct {
    const TLV_TYPE: u8 = 0x06;
}

impl TlvLength for VariableStruct {
    fn tlv_len(&self) -> usize {
        // value layout:
        // [u8: inner0_len][inner0 bytes][u8: inner1_len][inner1 elements u16...]
        1 + self.inner0.len() + 1 + (self.inner1.len() * 2)
    }

    fn tlv_len_is_constant() -> bool {
        false
    }
}

impl TlvMetadata for VariableStruct {}

impl DecodeTlvUnchecked for VariableStruct {
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        let _len_byte = buffer.get_tlv_length();
        Self::decode_tlv_value_unchecked(buffer)
    }
}

impl DecodeTlvValueUnchecked for VariableStruct {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();

        let inner0_len = buffer.get_u8() as usize;
        let mut inner0 = vec![0u8; inner0_len];
        buffer.copy_to_slice(&mut inner0);

        let inner1_len = buffer.get_u8() as usize;
        let mut inner1 = Vec::with_capacity(inner1_len);
        for _ in 0..inner1_len {
            inner1.push(buffer.get_u16());
        }

        VariableStruct { inner0, inner1 }
    }
}

impl TryEncodeTlv for VariableStruct {
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let num_bytes = write_tlv(buffer, Self::TLV_TYPE, self)?;
        Ok(num_bytes)
    }
}

impl TryEncodeTlvValue for VariableStruct {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;

        buffer.put_u8(self.inner0.len() as u8);
        buffer.put_slice(&self.inner0);
        buffer.put_u8(self.inner1.len() as u8);
        for v in &self.inner1 {
            buffer.put_u16(*v);
        }

        Ok(self.tlv_len())
    }
}
