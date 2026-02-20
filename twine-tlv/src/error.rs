// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TwineTlvError {
    #[error("Buffer was too short to begin decoding")]
    BufferDecodeTooShort,

    #[error("Unexpected length when parsing a TLV. Expected {0}; Found {1}")]
    BufferDecodeUnexpectedTlvLength(usize, usize),

    #[error("Buffer was too short to begin encoding")]
    BufferEncodeTooShort,

    #[error("Number of bytes exceeds buffer maximum length")]
    BufferMaxLength,

    #[error("TLV type mismatch")]
    BufferWrongType,

    #[error("Hex error: {0}")]
    HexError(#[from] faster_hex_thiserror::Error),
}
