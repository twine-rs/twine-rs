use clap::{Parser, Subcommand};

use otbr_client::{dbus::OtbrDbusClient, OtbrClient};

#[derive(Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a Thread network scan.
    Scan,
    /// Fetch a dataset from the Thread network.
    Dataset,
}

pub async fn run(args: Arguments) {
    let otbr = OtbrDbusClient::new().await.unwrap();

    match args.cmd {
        Commands::Scan => {
            let scan_results = otbr.scan().await.unwrap();
            for result in scan_results {
                println!("{:?}", result);
            }
        }
        Commands::Dataset => {
            let tlvs = otbr.active_dataset_tlvs().await.unwrap();
            println!("tlvs: {tlvs:02x?}");
        }
    }
}
