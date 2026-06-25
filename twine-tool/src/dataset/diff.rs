// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::Parser;
use twine_codec::decode_operational_dataset_item;

use super::parse;

#[derive(Debug, Parser)]
#[command(about = "Show the difference between two Thread operational datasets")]
pub struct Args {
    /// Base dataset as a TLV hex string
    pub base: String,

    /// Other dataset as a TLV hex string
    pub other: String,
}

pub fn run(args: Args) -> Result<()> {
    let base = parse::dataset(args.base.as_str())?;
    let other = parse::dataset(args.other.as_str())?;

    let diffs = base.diff(&other);

    if diffs.is_empty() {
        println!("No differences");
        return Ok(());
    }

    for entry in &diffs {
        if let Some(ref data) = entry.self_data {
            println!("- {}", decode_operational_dataset_item(data));
        }
        if let Some(ref data) = entry.other_data {
            println!("+ {}", decode_operational_dataset_item(data));
        }
    }

    Ok(())
}
