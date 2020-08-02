#![deny(unsafe_code)]
#![deny(warnings)]

use structopt::StructOpt;
use track::{Client, Config};

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
enum Command {
    Info(Info),
}

/// Retrives and prints tracking info for a parcel
#[derive(Debug, StructOpt)]
struct Info {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::from_args();

    match command {
        Command::Info(opts) => {
            let config = Config::load()?;
            let client = Client::new(config);
            client.track(&opts.tracking_number)?;
        }
    }

    Ok(())
}
