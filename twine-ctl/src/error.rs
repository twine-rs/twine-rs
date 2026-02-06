// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use thiserror::Error;
use twine_codec::TwineCodecError;

#[derive(Debug, Error)]
pub enum TwineCtlError {
    #[error("Command error: {0}")]
    CommandError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerialPortError(#[from] serialport::Error),

    #[error("Serial port read error")]
    SerialPortReadError,

    #[error("Not enough data returned from a shell command to parse response")]
    ShellParseError,

    #[error("Timeout waiting for response")]
    TimeoutError,

    #[error(transparent)]
    CodecError(#[from] TwineCodecError),

    #[error("Unexpected end of file")]
    UnexpectedEof,
}
