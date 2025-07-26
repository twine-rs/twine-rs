// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::Range;

use bytes::Buf;

use crate::{error::TwineTlvError, traits::TryEncodeTlv, GetTlvLength};

use super::{DecodeTlvUnchecked, TlvMetadata};

/// A type for working with a collection of TLVs.
#[derive(Clone, Copy, Debug)]
pub struct TlvCollection<const CAPACITY: usize> {
    /// Buffer space to hold TLV data up to a capacity of `CAPACITY` bytes.
    buffer: [u8; CAPACITY],
}

impl<const CAPACITY: usize> Default for TlvCollection<CAPACITY> {
    fn default() -> Self {
        Self {
            buffer: [0; CAPACITY],
        }
    }
}

impl<const CAPACITY: usize> TlvCollection<CAPACITY> {
    /// Takes ownership of an existing static buffer and creates a `TlvCollection` from it.
    pub fn new_from_static(buffer: [u8; CAPACITY]) -> Self {
        // todo: validate the data lengths inside the buffer
        Self { buffer }
    }

    #[cfg(any(test, feature = "alloc"))]
    /// Creates a new [`TlvCollection`] from a &[`str`].
    pub fn new_from_str(collection: &str) -> Result<Self, TwineTlvError> {
        let mut buffer = [0_u8; CAPACITY];
        let src_len = collection.as_bytes().len();
        faster_hex::hex_decode(collection.as_bytes(), &mut buffer[..src_len / 2])
            .map_err(TwineTlvError::HexError)?;

        Ok(Self { buffer })
    }

    /// Returns the length of the TLV data in the buffer.
    pub fn len(&self) -> usize {
        Self::find_buffer_len(self.buffer)
    }

    /// Returns the number of TLV items in the collection.
    pub fn count(&self) -> usize {
        let mut count = 0;
        let mut cursor = 0;

        while cursor < self.len() {
            if Self::peek_tlv_len(&self.buffer[cursor..]) == 0 {
                break;
            } else {
                count += 1;
                cursor += Self::next_tlv_position(&self.buffer[cursor..]);
            }
        }

        count
    }

    /// Search the buffer and determine the length of all TLV data
    fn find_buffer_len(buffer: impl AsRef<[u8]>) -> usize {
        let buffer = buffer.as_ref();
        let mut cursor = 0;

        while cursor < buffer.len() {
            if Self::peek_tlv_len(&buffer[cursor..]) == 0 {
                return cursor;
            } else {
                cursor = Self::next_tlv_position(&buffer[cursor..]) + cursor;
            }
        }

        buffer.len()
    }

    /// Peek into the TLV to determine the value length
    ///
    /// Note:
    ///
    /// It is the responsibility of the calling function to make sure the input
    /// buffer is aligned to the start of a TLV.
    pub(crate) fn peek_tlv_len(buffer: impl AsRef<[u8]>) -> usize {
        let mut buffer = buffer.as_ref();
        let _type_byte = buffer.get_u8();
        buffer.get_tlv_length()
    }

    /// Find the starting position of the next TLV in the buffer.
    fn next_tlv_position(buffer: impl AsRef<[u8]>) -> usize {
        let mut buffer = buffer.as_ref();
        let _start_byte = buffer.get_u8();
        let start_len = buffer.get_u8();

        let extended_len = if start_len == 0xFF {
            Some(buffer.get_u16())
        } else {
            None
        };

        let pos = if let Some(len) = extended_len {
            len + 4
        } else {
            (start_len + 2) as u16
        };

        pos as usize
    }

    pub fn contains_tlv<T>(&self, tlv_type: T) -> bool
    where
        T: Into<u8> + Copy,
    {
        self.find_tlv(tlv_type).is_some()
    }

    /// Search the collection for a TLV with a given type.
    ///
    /// Returns a byte slice that encapsulates the entire TLV (including type and length).
    pub fn find_tlv<T>(&self, tlv_type: T) -> Option<&[u8]>
    where
        T: Into<u8> + Copy,
    {
        if let Some(range) = Self::find_tlv_with_type(tlv_type.into(), self.buffer) {
            Some(&self.buffer[range])
        } else {
            None
        }
    }

    /// Search the buffer for a TLV with a given Type.
    ///
    /// Returns a [`Range`] that encapsulates the start and end of the entire TLV.
    fn find_tlv_with_type<T>(tlv_type: T, buffer: impl AsRef<[u8]>) -> Option<Range<usize>>
    where
        T: Into<u8> + Copy,
    {
        let buffer = buffer.as_ref();
        let mut cursor = 0;
        let len = Self::find_buffer_len(buffer);

        while cursor < len {
            let buffer_tlv_type = buffer[cursor];
            let next_pos = Self::next_tlv_position(&buffer[cursor..]) + cursor;

            if buffer_tlv_type == tlv_type.into() {
                // If matching against a TLV that is 0, check to see if this is a TLV
                // or if the end of the buffer has been reached
                if Self::peek_tlv_len(&buffer[cursor..]) == 0 {
                    return None;
                } else {
                    return Some(cursor..next_pos);
                }
            }

            cursor = next_pos
        }

        None
    }

    /// Remove the TLV data in a buffer at the specified buffer location, then compact the data.
    fn remove_data_and_compact_buffer(buffer: &mut impl AsMut<[u8]>, range: Range<usize>) {
        let buffer = buffer.as_mut();
        let mut scratch = [0_u8; CAPACITY];

        // Copy the data after the target TLV into the scratch buffer
        for i in range.end..buffer.len() {
            scratch[i - range.end] = buffer[i];
        }

        // Copy the scratch buffer back into the main buffer and include extra zeroed out data
        // from the scratch buffer
        for i in range.start..buffer.len() {
            buffer[i] = scratch[i - range.start];
        }
    }

    /// Decode a TLV from the buffer with type `T`.
    ///
    /// Note:
    /// * The buffer is expected to be already validated.
    /// * Decodes the first TLV of type `T` found in the buffer.
    pub fn decode_type_unchecked<T>(&self) -> Option<T>
    where
        T: DecodeTlvUnchecked + TlvMetadata + Copy,
    {
        if let Some(range) = Self::find_tlv_with_type(T::TLV_TYPE, self.buffer) {
            let result = T::decode_tlv_unchecked(&self.buffer[range]);
            Some(result)
        } else {
            None
        }
    }

    /// Attempt to append a TLV to the end of the collection.
    pub fn push<T>(&mut self, tlv: T) -> Result<usize, TwineTlvError>
    where
        T: TryEncodeTlv + TlvMetadata,
    {
        let len = self.len();
        if len + tlv.tlv_len() > CAPACITY {
            return Err(TwineTlvError::BufferMaxLength);
        }

        tlv.try_encode_tlv(&mut self.buffer[len..])
    }

    /// Remove the first TLV of type `T` from the collection, shifting all
    /// elements after it to the left.
    pub fn remove<T>(&mut self)
    where
        T: TlvMetadata,
    {
        if let Some(range) = Self::find_tlv_with_type(T::TLV_TYPE, self.buffer) {
            Self::remove_data_and_compact_buffer(&mut self.buffer, range);
        }
    }

    /// Replace the first TLV of type `T` in the collection with a new TLV.
    ///
    /// If the TLV is of a constant length, it will overwrite the existing TLV in place. If
    /// the TLV is of variable length, it will remove the existing TLV and append the new one, shifting
    /// the remaining elements to the left.
    pub fn replace<T>(&mut self, tlv: T) -> Result<(), TwineTlvError>
    where
        T: TryEncodeTlv + TlvMetadata,
    {
        if let Some(range) = Self::find_tlv_with_type(T::TLV_TYPE, self.buffer) {
            if T::tlv_len_is_constant() {
                let buf = &mut self.buffer[range.start..];
                let _ = tlv.try_encode_tlv(buf)?;
            } else {
                Self::remove_data_and_compact_buffer(&mut self.buffer, range.clone());
                self.push(tlv)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, vec};

    use crate::test_utils::{
        test_tlv_extended_data_type, TestTlvData, TestTlvDataTypeZero, TestTlvExtendedDataType,
        TestTlvVariableDataType, TEST_TLV_DATA, TEST_TLV_DATA_TYPE_ZERO,
    };

    use super::*;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn success_find_tlv_type() {
        let buffer = [
            0x01, 0x02, 0x03, 0x04, 0x02, 0xFF, 0x00, 0x02, 0xAA, 0xAA, 0x00, 0x03, 0x00, 0x00,
            0x16,
        ];
        let tlv = TlvCollection::<15>::find_tlv_with_type(0, buffer);
        assert_eq!(tlv, Some(Range { start: 10, end: 15 }));
        let bytes = &buffer[tlv.unwrap()];
        insta::assert_debug_snapshot!(bytes);
    }

    #[test]
    fn success_find_tlv_type_returns_none_unknown_tlv() {
        let buffer = [
            0x01, 0x02, 0x03, 0x04, 0x02, 0xFF, 0x00, 0x02, 0xAA, 0xAA, 0x00, 0x03, 0x00, 0x00,
            0x16,
        ];
        let tlv = TlvCollection::<15>::find_tlv_with_type(0xFF, buffer);
        assert_eq!(tlv, None);
    }

    #[test]
    fn success_find_tlv_type_returns_none_on_empty_buffer() {
        log_init();
        let buffer = [0_u8; 15];
        let tlv = TlvCollection::<15>::find_tlv_with_type(0xFF, buffer);
        assert_eq!(tlv, None);
    }

    #[test]
    fn success_len() {
        const CAPACITY: usize = 254;

        let mut buffer = [0_u8; CAPACITY];
        buffer[0] = 0x01;
        buffer[1] = 0x02;
        buffer[2] = 0x03;
        buffer[3] = 0x04;

        let tlv_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let len_test = tlv_data.len();
        assert_eq!(len_test, 4);
    }

    #[test]
    fn success_len_completely_full() {
        let mut buffer = [0_u8; 8];
        buffer[0] = 0x01;
        buffer[1] = 0x02;
        buffer[2] = 0x03;
        buffer[3] = 0x04;
        buffer[4] = 0x01;
        buffer[5] = 0x02;
        buffer[6] = 0x03;
        buffer[7] = 0x04;

        let tlv_data = TlvCollection::<8>::new_from_static(buffer);
        let len_test = tlv_data.len();
        assert_eq!(len_test, 8);
    }

    #[test]
    fn success_len_with_zero_tlv_at_end() {
        const CAPACITY: usize = 254;

        let mut buffer = [0_u8; CAPACITY];
        buffer[0] = 0x01;
        buffer[1] = 0x02;
        buffer[2] = 0x03;
        buffer[3] = 0x04;
        buffer[4] = 0x01;
        buffer[5] = 0x02;
        buffer[6] = 0x03;
        buffer[7] = 0x04;
        buffer[8] = 0x00;
        buffer[9] = 0x03;
        buffer[10] = 0x00;
        buffer[11] = 0x00;
        buffer[12] = 0x16;

        let tlv_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let len_test = tlv_data.len();
        assert_eq!(len_test, 13);
    }

    #[test]
    fn success_len_empty_buffer() {
        const CAPACITY: usize = 254;
        let buffer = [0_u8; CAPACITY];
        let tlv_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let len_test = tlv_data.len();
        assert_eq!(len_test, 0);
    }

    #[test]
    fn success_decode_type_not_first_tlv() {
        const CAPACITY: usize = 512;

        let mut buffer = [0_u8; CAPACITY];
        let extended_data = test_tlv_extended_data_type();
        let extended_len = extended_data.len();
        buffer[..extended_len].copy_from_slice(&extended_data);
        buffer[extended_len..extended_len + TEST_TLV_DATA.len()].copy_from_slice(&TEST_TLV_DATA);

        let test_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let tlv_data = test_data.decode_type_unchecked::<TestTlvData>();

        assert!(tlv_data.is_some());
        insta::assert_debug_snapshot!(tlv_data.unwrap());
    }

    #[test]
    fn success_decode_type_with_extended_len() {
        const CAPACITY: usize = 512;

        let mut buffer = [0_u8; CAPACITY];
        let extended_data = test_tlv_extended_data_type();
        let extended_len = extended_data.len();
        buffer[..extended_len].copy_from_slice(&extended_data);
        buffer[extended_len..extended_len + TEST_TLV_DATA.len()].copy_from_slice(&TEST_TLV_DATA);

        let test_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let tlv_data = test_data.decode_type_unchecked::<TestTlvExtendedDataType>();

        assert!(tlv_data.is_some());
        insta::assert_debug_snapshot!(tlv_data.unwrap());
    }

    #[test]
    fn success_decode_type_with_zero_tlv_in_first_pos() {
        const CAPACITY: usize = 512;

        let mut buffer = [0_u8; CAPACITY];
        let type_zero_data = TEST_TLV_DATA_TYPE_ZERO;
        buffer[..type_zero_data.len()].copy_from_slice(&type_zero_data);

        let test_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let tlv_data = test_data.decode_type_unchecked::<TestTlvDataTypeZero>();

        assert!(tlv_data.is_some());
        insta::assert_debug_snapshot!(tlv_data.unwrap());
    }

    #[test]
    fn fail_decode_type_not_in_buffer() {
        const CAPACITY: usize = 512;

        let mut buffer = [0_u8; CAPACITY];
        let extended_data = test_tlv_extended_data_type();
        let extended_len = extended_data.len();
        buffer[..extended_len].copy_from_slice(&extended_data);

        let test_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let tlv_data = test_data.decode_type_unchecked::<TestTlvData>();

        assert!(tlv_data.is_none());
    }

    #[test]
    fn fail_decode_type_zero_tlv_in_empty_buffer() {
        const CAPACITY: usize = 512;

        let buffer = [0_u8; CAPACITY];

        let test_data = TlvCollection::<CAPACITY>::new_from_static(buffer);
        let tlv_data = test_data.decode_type_unchecked::<TestTlvDataTypeZero>();

        assert!(tlv_data.is_none());
    }

    #[test]
    fn success_try_encode_tlv() {
        const CAPACITY: usize = 16;

        let tlv_data = TestTlvData([0xAA, 0xBB, 0xCC, 0xDD]);
        let mut tlv_collection = TlvCollection::<CAPACITY>::default();
        tlv_collection
            .push(tlv_data)
            .expect("Could not push TLV to collection");

        assert_eq!(tlv_collection.len(), 6);
        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }

    #[test]
    fn success_try_encode_multiple_tlv() {
        log_init();
        const CAPACITY: usize = 16;

        let tlv_data = TestTlvData([0xAA, 0xBB, 0xCC, 0xDD]);
        let mut tlv_collection = TlvCollection::<CAPACITY>::default();
        tlv_collection
            .push(tlv_data)
            .expect("Could not push TLV to collection");
        tlv_collection
            .push(tlv_data)
            .expect("Could not push TLV to collection");

        log::debug!("Buffer: {:?}", tlv_collection.buffer);

        assert_eq!(tlv_collection.len(), 12);
        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }

    #[test]
    fn success_remove_tlv() {
        const CAPACITY: usize = 16;

        let tlv_data = [
            0x02, 0x02, 0xAA, 0xAA, 0x01, 0x04, 0xAA, 0xBB, 0xCC, 0xDD, 0x03, 0x02, 0xBB, 0xBB,
            0x00, 0x00,
        ];

        let mut tlv_collection = TlvCollection::<CAPACITY>::new_from_static(tlv_data);
        tlv_collection.remove::<TestTlvData>();

        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }

    #[test]
    fn success_replace_same_len_tlv() {
        const CAPACITY: usize = 8;

        let tlv_data = [0x01, 0x04, 0xAA, 0xAA, 0xAA, 0xAA, 0x00, 0x00];

        let mut tlv_collection = TlvCollection::<CAPACITY>::new_from_static(tlv_data);
        let new_tlv = TestTlvData([0xAA, 0xBB, 0xCC, 0xDD]);
        tlv_collection
            .replace(new_tlv)
            .expect("Could not replace TLV");

        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }

    #[test]
    fn success_replace_different_len_tlv() {
        const CAPACITY: usize = 22;

        let tlv_data = [
            0x03, 0x02, 0xDE, 0xAD, 0x02, 0xFF, 0x00, 0x02, 0xAA, 0xAA, 0x01, 0x04, 0xAA, 0xBB,
            0xCC, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let mut tlv_collection = TlvCollection::<CAPACITY>::new_from_static(tlv_data);
        let new_tlv = TestTlvVariableDataType(vec![0xAA, 0xBB, 0xCC]);
        tlv_collection
            .replace(new_tlv)
            .expect("Could not replace TLV");

        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }

    #[test]
    fn fail_append_tlv_larger_than_remaining_capacity() {
        const CAPACITY: usize = 8;

        let tlv_data = [0x01, 0x04, 0xAA, 0xAA, 0xAA, 0xAA, 0x00, 0x00];

        let mut tlv_collection = TlvCollection::<CAPACITY>::new_from_static(tlv_data);
        let new_tlv = TestTlvData([0xAA, 0xBB, 0xCC, 0xDD]);

        let res = tlv_collection.push(new_tlv);
        assert_eq!(res, Err(TwineTlvError::BufferMaxLength));
    }

    #[test]
    fn success_count_tlvs() {
        const CAPACITY: usize = 16;

        let tlv_data = [
            0x02, 0x02, 0xAA, 0xAA, 0x01, 0x04, 0xAA, 0xBB, 0xCC, 0xDD, 0x03, 0x02, 0xBB, 0xBB,
            0x00, 0x00,
        ];
        let tlv_collection = TlvCollection::<CAPACITY>::new_from_static(tlv_data);

        let count = tlv_collection.count();
        assert_eq!(count, 3);
    }

    #[test]
    fn success_from_str() {
        log_init();
        const CAPACITY: usize = 32;

        let tlv_str = "0104AABBCCDD";
        let tlv_collection = TlvCollection::<CAPACITY>::new_from_str(tlv_str).unwrap();

        assert_eq!(tlv_collection.len(), 6);
        assert_eq!(tlv_collection.count(), 1);
        insta::assert_snapshot!(format!("{:02X?}", tlv_collection.buffer));
    }
}
