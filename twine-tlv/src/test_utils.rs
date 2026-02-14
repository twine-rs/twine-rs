// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use alloc::vec::Vec;
use bytes::{Buf, BufMut};

use twine_macros::Tlv;

use crate::{traits::TlvLength, DecodeTlvValueUnchecked, TryEncodeTlvValue, TwineTlvError};

pub(crate) const TEST_TLV_DATA_TYPE_ZERO: [u8; 3] = [0x00, 0x01, 0xAA];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x00, tlv_length = 1)]
pub(crate) struct TestTlvDataTypeZero([u8; 1]);

impl DecodeTlvValueUnchecked for TestTlvDataTypeZero {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let data = [buffer.get_u8()];
        TestTlvDataTypeZero(data)
    }
}

impl TryEncodeTlvValue for TestTlvDataTypeZero {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;

        if buffer.len() < self.tlv_len() {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        buffer.put_u8(self.0[0]);

        Ok(self.tlv_len())
    }
}

pub(crate) const TEST_TLV_DATA: [u8; 6] = [0x01, 0x04, 0xAA, 0xBB, 0xCC, 0xDD];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x01, tlv_length = 4)]
pub(crate) struct TestTlvData(pub(crate) [u8; 4]);

impl DecodeTlvValueUnchecked for TestTlvData {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let data = [
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
        ];
        TestTlvData(data)
    }
}

impl TryEncodeTlvValue for TestTlvData {
    fn try_encode_tlv_value(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, crate::error::TwineTlvError> {
        let mut buffer = buffer;

        buffer.put_u8(self.0[0]);
        buffer.put_u8(self.0[1]);
        buffer.put_u8(self.0[2]);
        buffer.put_u8(self.0[3]);

        Ok(self.tlv_len())
    }
}

pub(crate) fn test_tlv_extended_data_type() -> [u8; 256 + 4] {
    let mut data = [0; 256 + 4];
    data[0] = 0x02;
    data[1] = 0xFF;
    data[2] = 0x01;
    data[3] = 0x00;
    for byte in data[4..].iter_mut() {
        *byte = 0xAA_u8;
    }
    data
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x02, tlv_length = 256)]
pub(crate) struct TestTlvExtendedDataType([u8; 256]);

impl DecodeTlvValueUnchecked for TestTlvExtendedDataType {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let mut data = [0_u8; 256];

        for byte in data.iter_mut() {
            *byte = buffer.get_u8();
        }

        TestTlvExtendedDataType(data)
    }
}

impl TryEncodeTlvValue for TestTlvExtendedDataType {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;

        if buffer.len() < self.tlv_len() {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        for &byte in &self.0 {
            buffer.put_u8(byte);
        }

        Ok(self.tlv_len())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x03)]
pub(crate) struct TestTlvVariableDataType(pub(crate) Vec<u8>);

impl TlvLength for TestTlvVariableDataType {
    fn tlv_len(&self) -> usize {
        self.0.len()
    }
}

impl DecodeTlvValueUnchecked for TestTlvVariableDataType {
    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
        let mut buffer = buffer.as_ref();
        let mut data = Vec::new();

        while buffer.has_remaining() {
            data.push(buffer.get_u8());
        }

        TestTlvVariableDataType(data)
    }
}

impl TryEncodeTlvValue for TestTlvVariableDataType {
    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
        let mut buffer = buffer;

        if buffer.len() < self.tlv_len() {
            return Err(TwineTlvError::BufferEncodeTooShort);
        }

        for &byte in &self.0 {
            buffer.put_u8(byte);
        }

        Ok(self.tlv_len())
    }
}
