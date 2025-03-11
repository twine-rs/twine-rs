use clap::{Parser, Subcommand};
use tokio;
use zbus::Result;

mod otbr;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Interact with the OpenThread Border Router over D-Bus
    Dbus(otbr::dbus::Arguments),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Dbus(args) => {
            otbr::dbus::run(args).await;
        }
    }

    Ok(())
}
