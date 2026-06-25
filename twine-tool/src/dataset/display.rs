// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::Parser;

use super::parse;

#[derive(Debug, Parser)]
#[command(about = "Display a Thread operational dataset")]
pub struct Args {
    /// Input dataset as a TLV hex string (or pipe via stdin)
    pub dataset: Option<String>,

    /// Output as a comma-separated u8 array
    #[arg(long)]
    pub array: bool,
}

pub fn run(args: Args) -> Result<()> {
    let dataset = parse::dataset_from_arg_or_stdin(args.dataset.as_deref())?;

    if args.array {
        let bytes = hex::decode(dataset.as_hex_string())?;
        let formatted: Vec<String> = bytes.iter().map(|b| format!("0x{b:02x}")).collect();
        println!("[{}]", formatted.join(", "));
    } else {
        print!("{dataset}");
    }

    Ok(())
}
