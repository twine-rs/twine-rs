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
    #[arg(short = 'l', long, default_value_t = 10)]
    loops: usize,
}

#[tokio::test]
async fn batch_loop() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

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

    for loop_idx in 0..args.loops {
        log::info!("Starting loop {}/{}", loop_idx + 1, args.loops);

        let dataset = if let Some(ref ds) = args.dataset {
            log::info!("Loading dataset from user input");
            OperationalDataset::from_str(&ds).unwrap()
        } else {
            log::info!("Generating random dataset");
            OperationalDataset::random().unwrap()
        };

        log::info!("Using Dataset: {}", dataset.as_hex_string());

        for port in &matching_ports {
            log::info!("Found device on port: {}", port.port_name);

            let mut dev = match TwineCtlSerialShell::open(&port.port_name, 115200).await {
                Ok(dev) => dev,
                Err(e) => {
                    panic!("Failed to open serial port: {e}");
                }
            };

            let version = dev.version().await.unwrap();
            log::info!("Version: {}", version);

            dev.reset().await.unwrap_or_else(|e| {
                panic!("Failed to reset device: {e}");
            });

            dev.factory_reset().await.unwrap_or_else(|e| {
                panic!("Failed to factory reset device: {e}");
            });

            assert!(dev.active_dataset().await.is_err());

            log::info!("Setting dataset...");

            dev.attach_with_dataset(&dataset).await.unwrap_or_else(|e| {
                panic!("Failed to set active dataset: {e}");
            });

            match dev.active_dataset().await {
                Ok(active_dataset) => {
                    assert_eq!(active_dataset.as_hex_string(), dataset.as_hex_string());
                    log::info!("{active_dataset:?}")
                }
                Err(e) => panic!("Failed to fetch active dataset: {e}"),
            }
        }
    }
}
