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
//! For constant length TLVs, the [`ConstantTlv`](twine_macros::ConstantTlv) macro can be used to
//! generate the necessary trait implementations automatically.
//! 
//! For convience, the [`ConstantTlv`](twine_macros::ConstantTlv) and [`Tlv`](twine_macros::Tlv)
//! macros can be used to generate the necessary boilerplate implementations automatically. While
//! using these macros, the only traits that need to be implemented are
//! [`DecodeTlvValueUnchecked`](traits::DecodeTlvValueUnchecked),
//! [`TryEncodeTlvValue`](traits::TryEncodeTlvValue), and potentially [`TlvLength](traits::TlvLength)
//! if the TLV has a variable length.

#![no_std]

#[cfg(test)]
extern crate alloc;

use bytes::Buf;

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
        return Err(TwineTlvError::BufferTlvWrongType);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        test_tlv_extended_data_type, TestTlvData, TestTlvExtendedDataType, TEST_TLV_DATA,
    };

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
}
