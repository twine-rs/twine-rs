use std::str::FromStr;

use clap::Parser;
use twine_codec::OperationalDataset;

#[derive(Debug, Parser)]
#[command(version, about = "Parse and print a Thread operational dataset")]
struct Args {
    #[arg(help = "Dataset hex string (optional 0x prefix; whitespace is ignored)")]
    dataset: String,
}

fn normalize_dataset(input: &str) -> String {
    let trimmed = input.trim();
    let without_prefix = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    without_prefix
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect()
}

fn main() {
    let args = Args::parse();
    let dataset_hex = normalize_dataset(&args.dataset);

    if dataset_hex.is_empty() {
        eprintln!("Dataset input is empty");
        std::process::exit(2);
    }

    let dataset = OperationalDataset::from_str(&dataset_hex).unwrap_or_else(|error| {
        eprintln!("Invalid dataset: {error}");
        std::process::exit(2);
    });

    println!("Dataset Hex: {dataset_hex}");
    println!("---");
    print!("{dataset}");
}
