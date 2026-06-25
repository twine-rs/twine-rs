// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::{Parser, Subcommand};

mod diff;
mod display;
mod generate;
mod modify;
pub(crate) mod parse;

#[derive(Debug, Parser)]
#[command(about = "Generate, display, or modify a Thread operational dataset")]
pub struct Args {
    #[command(subcommand)]
    command: DatasetSubcommand,
}

#[derive(Debug, Subcommand)]
enum DatasetSubcommand {
    /// Show the difference between two Thread operational datasets
    Diff(diff::Args),
    /// Display a Thread operational dataset
    Display(display::Args),
    /// Generate a Thread operational dataset from JSON
    Generate(generate::Args),
    /// Modify an existing operational dataset
    Modify(Box<modify::Args>),
}

pub fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        DatasetSubcommand::Diff(args) => diff::run(args),
        DatasetSubcommand::Display(args) => display::run(args),
        DatasetSubcommand::Generate(args) => generate::run(args),
        DatasetSubcommand::Modify(args) => modify::run(*args),
    }
}
