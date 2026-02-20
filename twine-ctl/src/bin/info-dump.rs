use twine_ctl::{TwineCtl, TwineCtlSerialShell};

#[tokio::main]
async fn main() {
    env_logger::init();

    let port_name = "/dev/tty.usbmodemCDF5EFD1CFD41";
    let baud_rate = 115200;

    let mut dev = match TwineCtlSerialShell::open(port_name, baud_rate).await {
        Ok(dev) => dev,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            return;
        }
    };

    match dev.version().await {
        Ok(version) => {
            println!("Version:\t{version}");
        }
        Err(e) => {
            eprintln!("Failed to get device version: {e}");
        }
    }

    match dev.uptime().await {
        Ok(uptime) => {
            println!("Uptime:\t\t{uptime}");
        }
        Err(e) => {
            eprintln!("Failed to get device uptime: {e}");
        }
    }

    match dev.role().await {
        Ok(role) => {
            println!("Role:\t\t{role:?}");
        }
        Err(e) => {
            eprintln!("Failed to get device role: {e}");
        }
    }

    match dev.network_name().await {
        Ok(name) => {
            println!("Network name:\t{name}");
        }
        Err(e) => {
            eprintln!("Failed to get network name: {e}");
        }
    }

    match dev.pan_id().await {
        Ok(pan_id) => {
            println!("PAN ID:\t\t{pan_id}");
        }
        Err(e) => {
            eprintln!("Failed to get PAN ID: {e}");
        }
    }

    match dev.channel().await {
        Ok(channel) => {
            println!("Channel:\t{}", channel.channel());
        }
        Err(e) => {
            eprintln!("Failed to get Channel: {e}");
        }
    }

    match dev.preferred_channel_mask().await {
        Ok(channel_mask) => {
            println!("Preferred:\t0x{:08x}", channel_mask.mask());
        }
        Err(e) => {
            eprintln!("Failed to get Channel Mask: {e}");
        }
    }

    match dev.supported_channel_mask().await {
        Ok(channel_mask) => {
            println!("Supported:\t0x{:08x}", channel_mask.mask());
        }
        Err(e) => {
            eprintln!("Failed to get Supported Channel Mask: {e}");
        }
    }

    match dev.rloc16().await {
        Ok(rloc16) => {
            println!("RLOC16:\t\t{rloc16}");
        }
        Err(e) => {
            eprintln!("Failed to get RLOC16: {e}");
        }
    }

    match dev.active_dataset().await {
        Ok(dataset) => {
            dataset.pretty_fmt();
        }
        Err(e) => {
            eprintln!("Failed to get Active Dataset: {e}");
        }
    }
}
