// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::{Buf, BufMut};

use crate::error::TwineTlvError;

macro_rules! encoded_len {
    ($len:expr) => {{
        let len = $len;
        if len < 0xFF {
            len + 2 // Type + Length
        } else {
            len + 4 // Type + Extended Length
        }
    }};
}

pub trait PutTlvLength: BufMut {
    fn put_tlv_length(&mut self, length: usize);
}

impl PutTlvLength for &mut [u8] {
    fn put_tlv_length(&mut self, length: usize) {
        if length < 0xFF {
            self.put_u8(length as u8);
        } else {
            self.put_u8(0xFF);
            self.put_u16(length as u16);
        }
    }
}

pub trait GetTlvLength: Buf {
    /// Extract the length of a TLV from the buffer
    ///
    /// Expects the next byte(s) to represent the length of the TLV.
    fn get_tlv_length(&mut self) -> usize;
}

impl GetTlvLength for &[u8] {
    fn get_tlv_length(&mut self) -> usize {
        let start_len = self.get_u8();

        let extended_len = if start_len == 0xFF {
            Some(self.get_u16())
        } else {
            None
        };

        let len = if let Some(len) = extended_len {
            len
        } else {
            start_len as u16
        };

        len as usize
    }
}

pub trait DecodeTlvUnchecked {
    /// Decode some data type using the TLV format, but skip validation checks.
    ///
    /// **Note**
    ///
    /// * The first byte should align with the TLV "type" byte.
    /// * Does not perform any validation checks on the data.
    fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self
    where
        Self: Sized;
}

pub trait DecodeTlvValueUnchecked {
    /// Decode the value portion of a TLV data type without validation.
    ///
    /// **Note**
    ///
    /// * The first byte should align with the TLV value bytes.
    /// * Does not perform any validation checks on the data.
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self
    where
        Self: Sized;
}

impl DecodeTlvValueUnchecked for u8 {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        buffer.get_u8()
    }
}

impl DecodeTlvValueUnchecked for u16 {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        buffer.get_u16()
    }
}

impl DecodeTlvValueUnchecked for u32 {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        buffer.get_u32()
    }
}

impl DecodeTlvValueUnchecked for u64 {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        buffer.get_u64()
    }
}

impl<const N: usize> DecodeTlvValueUnchecked for [u8; N] {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let mut array = [0_u8; N];

        for i in 0..buffer.len() {
            array[i] = buffer.get_u8();
        }
        array
    }
}

pub trait TryEncodeTlv: TryEncodeTlvValue {
    /// Encode some data type into the TLV format.
    ///
    /// Returns the number of bytes written to the buffer (including the type and length bytes).
    fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError>;
}

pub trait TryEncodeTlvValue {
    /// Encode the value portion of a TLV data type.
    ///
    /// Returns the number of bytes written to the buffer.
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError>;
}

impl TryEncodeTlvValue for u8 {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u8(*self);
        Ok(1)
    }
}

impl TryEncodeTlvValue for u16 {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u16(*self);
        Ok(2)
    }
}

impl TryEncodeTlvValue for u32 {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u32(*self);
        Ok(4)
    }
}

impl TryEncodeTlvValue for u64 {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();
        buffer.put_u64(*self);
        Ok(8)
    }
}

impl<const N: usize> TryEncodeTlvValue for [u8; N] {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer.as_mut();

        if buffer.len() < N {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        for &byte in self.iter() {
            buffer.put_u8(byte);
        }

        Ok(N)
    }
}

pub trait TlvType {
    /// The TLV type of the data.
    const TLV_TYPE: u8;
}

pub trait TlvLength {
    /// The length of the TLV value payload.
    fn tlv_len(&self) -> usize;

    /// Determine if the type has a constant length.
    fn tlv_len_is_constant() -> bool {
        false
    }

    /// The total length of the TLV, including type and length bytes.
    fn tlv_total_len(&self) -> usize {
        encoded_len!(self.tlv_len())
    }
}

/// Generic metadata for a type that can be represented as a TLV.
pub trait TlvMetadata: TlvType + TlvLength {}

/// Metadata for a TLV that will always have a constant, fixed length.
pub trait TlvConstantMetadata: TlvMetadata {
    /// Constant expected length for the TLV
    const TLV_LEN: usize;

    /// Constant expected length of the entire TLV (including type and length bytes)
    fn tlv_total_constant_len() -> usize {
        encoded_len!(Self::TLV_LEN)
    }
}
