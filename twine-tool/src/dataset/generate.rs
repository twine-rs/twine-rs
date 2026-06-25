// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    io::{self, IsTerminal, Read},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use serde::Deserialize;
use twine_codec::{
    Channel, ChannelMask, DelayTimer, ExtendedPanId, MeshLocalPrefix, NetworkKey, NetworkName,
    OperationalDataset, PanId, Pskc,
};

use super::parse;

#[derive(Debug, Parser)]
#[command(about = "Generate a Thread operational dataset from JSON")]
pub struct Args {
    /// Path to a JSON file (reads from stdin if omitted)
    pub input: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct DatasetJson {
    active_timestamp: Option<String>,
    pending_timestamp: Option<String>,
    channel: Option<u16>,
    channel_mask: Option<String>,
    extended_pan_id: Option<String>,
    mesh_local_prefix: Option<String>,
    network_key: Option<String>,
    network_name: Option<String>,
    pan_id: Option<String>,
    pskc: Option<String>,
    delay_timer: Option<u32>,
    security_policy: Option<String>,
}

fn apply_json(json: DatasetJson) -> Result<OperationalDataset> {
    let mut dataset = OperationalDataset::from_str("").context("failed to create dataset")?;

    if let Some(ref s) = json.active_timestamp {
        let ts = parse::timestamp(s).context("active_timestamp")?;
        dataset
            .set_active_timestamp(ts)
            .context("failed to set active timestamp")?;
    }

    if let Some(ref s) = json.pending_timestamp {
        let ts = parse::timestamp(s).context("pending_timestamp")?;
        dataset
            .set_pending_timestamp(ts)
            .context("failed to set pending timestamp")?;
    }

    if let Some(n) = json.channel {
        dataset
            .set_channel(Channel::new(0, n))
            .context("failed to set channel")?;
    }

    if let Some(ref s) = json.channel_mask {
        let mask = ChannelMask::from_str(s).context("failed to parse channel_mask")?;
        dataset
            .set_channel_mask(mask)
            .context("failed to set channel mask")?;
    }

    if let Some(ref s) = json.extended_pan_id {
        let bytes = parse::hex_bytes::<8>(s, "extended_pan_id")?;
        dataset
            .set_extended_pan_id(ExtendedPanId::from(bytes))
            .context("failed to set extended PAN ID")?;
    }

    if let Some(ref s) = json.mesh_local_prefix {
        let bytes = parse::hex_bytes::<8>(s, "mesh_local_prefix")?;
        dataset
            .set_mesh_local_prefix(MeshLocalPrefix::from(bytes))
            .context("failed to set mesh-local prefix")?;
    }

    if let Some(ref s) = json.network_key {
        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);
        let key = NetworkKey::from_str(s).context("failed to parse network_key")?;
        dataset
            .set_network_key(key)
            .context("failed to set network key")?;
    }

    if let Some(ref s) = json.network_name {
        let name = NetworkName::from_str(s).context("failed to parse network_name")?;
        dataset
            .set_network_name(name)
            .context("failed to set network name")?;
    }

    if let Some(ref s) = json.pan_id {
        let pan_id = PanId::from_str(s).context("failed to parse pan_id")?;
        dataset.set_pan_id(pan_id).context("failed to set PAN ID")?;
    }

    if let Some(ref s) = json.pskc {
        let bytes = parse::hex_bytes::<16>(s, "pskc")?;
        dataset
            .set_pskc(Pskc::from(bytes))
            .context("failed to set PSKc")?;
    }

    if let Some(millis) = json.delay_timer {
        dataset
            .set_delay_timer(DelayTimer::from(millis))
            .context("failed to set delay timer")?;
    }

    if let Some(ref s) = json.security_policy {
        let policy = parse::security_policy(s).context("security_policy")?;
        dataset
            .set_security_policy(policy)
            .context("failed to set security policy")?;
    }

    Ok(dataset)
}

pub fn run(args: Args) -> Result<()> {
    let json_str = match args.input {
        Some(path) => std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read '{}'", path.display()))?,
        None => {
            let stdin = io::stdin();
            if stdin.is_terminal() {
                bail!("no input provided: pass a file path or pipe JSON via stdin");
            }
            let mut buf = String::new();
            stdin
                .lock()
                .read_to_string(&mut buf)
                .context("failed to read stdin")?;
            buf
        }
    };

    let json: DatasetJson =
        serde_json::from_str(&json_str).context("failed to parse JSON input")?;
    let dataset = apply_json(json)?;
    println!("{}", dataset.as_hex_string());
    Ok(())
}
