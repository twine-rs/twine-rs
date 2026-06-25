// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::{Parser, Subcommand};

mod dataset;

#[derive(Debug, Parser)]
#[command(version, about = "Multi-use tool for interacting with Thread networks")]
struct TwineTool {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate, display, or modify a Thread operational dataset
    Dataset(dataset::Args),
}

fn main() -> anyhow::Result<()> {
    let cli = TwineTool::parse();

    match cli.command {
        Commands::Dataset(args) => dataset::run(args),
    }
}
