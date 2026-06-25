// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    io::{self, IsTerminal, Read},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use twine_codec::{Authoritative, OperationalDataset, Timestamp};

pub(crate) fn dataset(s: &str) -> Result<OperationalDataset> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    OperationalDataset::from_str(s).context("failed to parse input dataset")
}

pub(crate) fn dataset_from_arg_or_stdin(arg: Option<&str>) -> Result<OperationalDataset> {
    match arg {
        Some(s) => dataset(s),
        None => {
            let stdin = io::stdin();
            if stdin.is_terminal() {
                bail!("no dataset provided: pass as an argument or pipe via stdin");
            }
            let mut buf = String::new();
            stdin
                .lock()
                .read_to_string(&mut buf)
                .context("failed to read stdin")?;
            dataset(buf.trim())
        }
    }
}

pub(crate) fn timestamp(s: &str) -> Result<Timestamp> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        bail!("timestamp must be in format seconds:ticks:authoritative (1:0:false)");
    }

    let seconds: u64 = parts[0]
        .parse()
        .context("invalid timestamp seconds value")?;
    let ticks: u16 = parts[1].parse().context("invalid timestamp ticks value")?;
    let authoritative: bool = parts[2]
        .parse()
        .context("invalid timestamp authoritative value (expected true or false)")?;

    Ok(Timestamp::from((
        seconds,
        ticks,
        Authoritative(authoritative),
    )))
}

pub(crate) fn hex_bytes<const N: usize>(s: &str, name: &str) -> Result<[u8; N]> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);

    if s.len() != N * 2 {
        bail!(
            "{name} must be exactly {} hex characters, got {}",
            N * 2,
            s.len()
        );
    }

    let mut buf = [0u8; N];
    hex::decode_to_slice(s, &mut buf).context(format!("invalid hex for {name}"))?;
    Ok(buf)
}
