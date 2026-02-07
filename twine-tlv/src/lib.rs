// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `nostd` library for working with TLVs in a `static` collection.
//!
//! Similar to `heapless`, the goal of this library is to provide a way to work with collections of
//! TLVs in environments where dynamic memory allocation is either not available or not desired.
//! The primary use case is for use with the Thread networking protocol, but it is not limited to
//! that use case.
//!
//! Custom TLV types can be defined by implementing the [`TryEncodeTlv`](traits::TryEncodeTlv),
//! [`DecodeTlvUnchecked`](traits::DecodeTlvUnchecked), and [`TlvMetadata`](traits::TlvMetadata)
//! traits. Optionally, if the TLV has a constant length, the [`TlvConstantMetadata`] trait can also
//! be implemented.
//!
//! For convience, the [`Tlv`](twine_macros::Tlv) macro can be used to generate the necessary
//! boilerplate implementations automatically. While using this macro, the only traits that need to
//! be implemented are [`DecodeTlvValueUnchecked`](traits::DecodeTlvValueUnchecked),
//! [`TryEncodeTlvValue`](traits::TryEncodeTlvValue), and potentially [`TlvLength](traits::TlvLength)
//! if the TLV has a variable length.

#![no_std]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

use bytes::{Buf, BufMut};

mod collection;
pub mod error;
#[cfg(test)]
mod test_utils;
mod traits;

pub use collection::TlvCollection;
pub use error::TwineTlvError;
pub use traits::{
    DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, PutTlvLength, TlvConstantMetadata,
    TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue,
};

pub mod prelude {
    pub use bytes::Buf;

    pub use super::{
        write_tlv, DecodeTlvUnchecked, DecodeTlvValueUnchecked, GetTlvLength, TlvCollection,
        TlvConstantMetadata, TlvLength, TlvMetadata, TlvType, TryEncodeTlv, TryEncodeTlvValue,
        TwineTlvError,
    };
}

const TLV_EXTENDED_LEN_ID: u8 = 0xFF;

/// Validate a constant length TLV
///
/// Validates:
/// * Expected total length of the TLV (including type and length bytes)
/// * Expected Type byte
/// * Expected Length byte
///
/// Does not validate the contents of the "value" bytes
pub fn validate_const_len_tlv<T>(buffer: impl AsRef<[u8]>) -> Result<(), TwineTlvError>
where
    T: TlvConstantMetadata,
{
    let mut buffer = buffer.as_ref();

    if buffer.len() < T::tlv_total_constant_len() {
        return Err(TwineTlvError::BufferDecodeTooShort);
    }

    let type_byte = buffer.get_u8();
    if type_byte != T::TLV_TYPE {
        return Err(TwineTlvError::BufferWrongType);
    }

    let len = buffer.get_tlv_length();

    if len != T::TLV_LEN {
        return Err(TwineTlvError::BufferDecodeUnexpectedTlvLength(
            T::TLV_LEN,
            len,
        ));
    }

    Ok(())
}

/// Write a TLV to a buffer with the provided inputs.
pub fn write_tlv<T>(
    buffer: &mut [u8],
    tlv_type: impl Into<u8>,
    value: &T,
) -> Result<usize, TwineTlvError>
where
    T: TlvMetadata + TryEncodeTlvValue,
{
    let mut buffer = buffer;
    let value_bytes_len = value.tlv_len();

    if value_bytes_len > u16::MAX as usize {
        return Err(TwineTlvError::BufferMaxLength);
    }

    let original_remaining = buffer.remaining_mut();

    // Available space is the length of the buffer minus the type and length bytes
    let available_bytes = if value_bytes_len >= TLV_EXTENDED_LEN_ID as usize {
        // remaining - type byte - TLV_EXTENDED_LEN_ID - length high - length low
        original_remaining - 1 - 3
    } else {
        // remaining - type byte - length
        original_remaining - 1 - 1
    };

    if value_bytes_len > available_bytes {
        return Err(TwineTlvError::BufferEncodeTooShort);
    }

    buffer.put_u8(tlv_type.into());

    if value_bytes_len >= TLV_EXTENDED_LEN_ID as usize {
        buffer.put_u8(TLV_EXTENDED_LEN_ID);
        buffer.put_u16(value_bytes_len as u16);
    } else {
        buffer.put_u8(value_bytes_len as u8);
    }

    let value_bytes_written = value.try_encode_tlv_value(buffer)?;

    Ok(original_remaining - buffer.remaining_mut() + value_bytes_written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        test_tlv_extended_data_type, TestTlvData, TestTlvExtendedDataType, TEST_TLV_DATA,
    };
    use twine_macros::Tlv;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
    #[tlv(tlv_type = 0x00, tlv_length = 3)]
    struct ExampleTestData {
        foo: u8,
        bar: u16,
    }

    impl DecodeTlvValueUnchecked for ExampleTestData {
        fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
            let mut buffer = buffer.as_ref();
            let foo = buffer.get_u8();
            let bar = buffer.get_u16();
            ExampleTestData { foo, bar }
        }
    }

    impl TryEncodeTlvValue for ExampleTestData {
        fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
            let mut buffer = buffer;
            buffer.put_u8(self.foo);
            buffer.put_u16(self.bar);
            Ok(Self::TLV_LEN)
        }
    }

    impl IntoIterator for ExampleTestData {
        type Item = u8;
        type IntoIter = core::array::IntoIter<u8, { ExampleTestData::TLV_LEN }>;

        fn into_iter(self) -> Self::IntoIter {
            let bar = u16::to_be_bytes(self.bar);
            let bytes = [self.foo, bar[0], bar[1]];
            bytes.into_iter()
        }
    }

    #[test]
    fn success_validate_const_len_tlv() {
        let mut buffer = [0u8; 6];
        buffer.copy_from_slice(&TEST_TLV_DATA);

        assert!(validate_const_len_tlv::<TestTlvData>(&buffer).is_ok());
    }

    #[test]
    fn success_validate_const_extended_len_tlv() {
        let mut buffer = [0u8; 256 + 4];
        buffer.copy_from_slice(&test_tlv_extended_data_type());

        assert!(validate_const_len_tlv::<TestTlvExtendedDataType>(&buffer).is_ok());
    }

    #[test]
    fn fail_validate_const_len_tlv_total_len_too_short() {
        const TEST_BYTES_TOTAL_LEN_TOO_SHORT: [u8; 4] = [0x00, 0x03, 0x00, 0x00];
        let test = validate_const_len_tlv::<ExampleTestData>(&TEST_BYTES_TOTAL_LEN_TOO_SHORT);
        assert_eq!(test, Err(TwineTlvError::BufferDecodeTooShort));
    }

    #[test]
    fn fail_validate_const_len_tlv_wrong_type() {
        const TEST_BYTES_WRONG_TYPE: [u8; 5] = [0xFF, 0x03, 0x00, 0x00, 0x16];
        let test = validate_const_len_tlv::<ExampleTestData>(&TEST_BYTES_WRONG_TYPE);
        assert_eq!(test, Err(TwineTlvError::BufferWrongType));
    }

    #[test]
    fn fail_validate_const_len_tlv_len_too_short() {
        const TEST_BYTES_LEN_TOO_SHORT: [u8; 5] = [0x00, 0x02, 0x00, 0x00, 0x16];
        let test = validate_const_len_tlv::<ExampleTestData>(TEST_BYTES_LEN_TOO_SHORT);
        assert_eq!(
            test,
            Err(TwineTlvError::BufferDecodeUnexpectedTlvLength(3, 2))
        );
    }

    #[test]
    fn success_write_tlv() {
        // Standard length byte
        const TEST_BYTES: [u8; 5] = [0x00, 0x03, 0x00, 0x00, 0x16];
        const TEST_DATA: ExampleTestData = ExampleTestData {
            foo: 0x00,
            bar: 0x0016,
        };
        let mut test_buffer: [u8; 10] = [0; 10];
        let result =
            write_tlv::<ExampleTestData>(&mut test_buffer, ExampleTestData::TLV_TYPE, &TEST_DATA)
                .expect("Could not write TLV");
        assert_eq!(result, 5);
        assert_eq!(test_buffer[0..5], TEST_BYTES);
    }
}
