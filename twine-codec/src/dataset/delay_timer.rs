// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::Buf;

use twine_macros::Tlv;
use twine_tlv::prelude::*;

/// An unsigned 32-bit number representing the time delay before the pending
/// dataset to be applied, in milliseconds.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Tlv)]
#[tlv(tlv_type = 0x34, tlv_length = 4, derive_inner)]
pub struct DelayTimer(u32);
