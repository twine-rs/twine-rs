use std::str::FromStr;

use clap::Parser;
use env_logger::Env;
use serialport::{available_ports, SerialPortType};

use twine_codec::OperationalDataset;
use twine_ctl::{TwineCtl, TwineCtlSerialShell};

const NORDIC_VID: u16 = 0x1915;
const NORDIC_PID: u16 = 0xcafe;
const NORDIC_OPENTHREAD_PRODUCT: &str = "nRF528xx OpenThread Device";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dataset: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let dataset = if let Some(ds) = args.dataset {
        println!("Loading dataset from user input");
        OperationalDataset::from_str(&ds).unwrap()
    } else {
        println!("Generating random dataset");
        OperationalDataset::random().unwrap()
    };

    println!("Using Dataset: {}", dataset.as_hex_string());

    let matching_ports: Vec<_> = available_ports()
        .unwrap()
        .into_iter()
        .filter(|p| match &p.port_type {
            SerialPortType::UsbPort(info) => {
                #[cfg(target_os = "macos")]
                if p.port_name.starts_with("/dev/tty") {
                    return false;
                }

                info.vid == NORDIC_VID
                    && info.pid == NORDIC_PID
                    && info.product == Some(NORDIC_OPENTHREAD_PRODUCT.to_string())
            }
            _ => false,
        })
        .collect();

    for port in matching_ports {
        println!("Found device on port: {}", port.port_name);

        let mut dev = match TwineCtlSerialShell::open(&port.port_name, 115200).await {
            Ok(dev) => dev,
            Err(e) => {
                eprintln!("Failed to open serial port: {}", e);
                continue;
            }
        };

        let version = dev.version().await.unwrap();
        println!("Version: {}", version);

        dev.reset().await.unwrap_or_else(|e| {
            eprintln!("Failed to reset device: {}", e);
        });

        dev.factory_reset().await.unwrap_or_else(|e| {
            eprintln!("Failed to factory reset device: {}", e);
        });

        println!("Setting dataset...");

        dev.attach_with_dataset(&dataset).await.unwrap_or_else(|e| {
            eprintln!("Failed to set active dataset: {}", e);
        });

        let active_dataset = dev
            .active_dataset()
            .await
            .map_err(|e| {
                eprintln!("Failed to fetch active dataset: {}", e);
            })
            .unwrap();
        println!("{active_dataset}");
    }
}
