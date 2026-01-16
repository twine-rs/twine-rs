// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use thiserror::Error;

use twine_tlv::TwineTlvError;

#[derive(Debug, Error, PartialEq)]
pub enum TwineCodecError {
    #[error("Could not convert buffer of bytes to {0}")]
    BufferBytesConversion(&'static str),

    #[error("Buffer was too short to begin decoding")]
    BufferDecodeTooShort,

    #[error("Unexpected length when parsing a TLV. Expected {0}; Found {1}")]
    BufferDecodeUnexpectedTlvLength(usize, usize),

    #[error("No space left in the buffer to write data")]
    BufferEncodeNoSpace,

    #[error("Provided value length exceeds TLV maximum")]
    BufferEncodeMaxLength,

    #[error("Number of bytes exceeds buffer maximum length; expected {0}, found {1}")]
    BufferMaxLength(usize, usize),

    #[error("TLV type mismatch")]
    BufferTlvWrongType,

    #[error("Hex decode error")]
    HexDecodeError,

    #[error("Missing dataset parameter: {0}")]
    MissingDatasetParameter(&'static str),

    #[error("String parse error")]
    StringParseError,

    #[error(transparent)]
    TlvError(#[from] TwineTlvError),

    #[error("Could not build {0}")]
    TypeBuildError(&'static str),

    #[error("{0}")]
    Internal(&'static str),
}
