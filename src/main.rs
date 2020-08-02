#![deny(unsafe_code)]
#![deny(warnings)]

use std::io::{self, Write};
use structopt::StructOpt;
use track::{Client, Config};

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
enum Command {
    /// Initialises the configuration
    Init,

    /// Retrives and prints tracking info for a parcel
    Info(Info),
}

#[derive(Debug, StructOpt)]
struct Info {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Command::from_args() {
        Command::Init => {
            let mut input = String::new();

            print!("Enter your La Poste API key: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut input)?;

            let config = Config::new(input.trim());
            config.save()?;

            println!("The configuration has been initialised.");
        }

        Command::Info(opts) => {
            let config = Config::load()?;
            let client = Client::new(config);
            client.track(&opts.tracking_number)?;
        }
    }

    Ok(())
}
