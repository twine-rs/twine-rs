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
use twine_codec::{Authoritative, OperationalDataset, SecurityPolicy, Timestamp};

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

/// Parse a security policy string in display format: `"672 onrc 0"`.
///
/// The format is `<rotation_hours> <flags> <version_threshold>` where flags
/// are a subset of `onrcCepLR` representing the enabled policy bits.
pub(crate) fn security_policy(s: &str) -> Result<SecurityPolicy> {
    let parts: Vec<&str> = s.splitn(3, ' ').collect();
    if parts.len() != 3 {
        bail!("security policy must be in format 'HOURS FLAGS VERSION' ('672 onrc 0')");
    }

    let rotation_hours: u16 = parts[0]
        .parse()
        .context("invalid security policy rotation time hours")?;
    let flags = parts[1];
    let version: u8 = parts[2]
        .parse()
        .context("invalid security policy version threshold")?;

    let mut policy = SecurityPolicy(0);
    policy.set_rotation_time_hours(rotation_hours);
    policy.set_obtain_network_key(flags.contains('o'));
    policy.set_native_commissioning(flags.contains('n'));
    policy.set_legacy_routers(flags.contains('r'));
    policy.set_external_commissioner(flags.contains('c'));
    policy.set_commercial_commissioning(flags.contains('C'));
    policy.set_autonomous_enrollment(flags.contains('e'));
    policy.set_network_key_provisioning(flags.contains('p'));
    policy.set_to_ble_link(flags.contains('L'));
    policy.set_non_ccm_routers(flags.contains('R'));
    policy.set_version_threshold_raw(version);

    Ok(policy)
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
