use clap::Parser;
use miniscript::bitcoin;

mod main_cli;
mod main_gui;
mod styles;
mod util;

#[derive(Parser, Debug)]
#[command(name = "spk_recovery")]
#[command(about = "SPK Recovery Tool - scan and recover Bitcoin from descriptors", long_about = None)]
struct CliArgs {
    /// Run in CLI mode (otherwise runs GUI)
    #[arg(long)]
    cli: bool,
}

const NETWORK: bitcoin::Network = bitcoin::Network::Regtest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    if args.cli {
        main_cli::run()?;
    } else {
        main_gui::run()?;
    }

    Ok(())
}
