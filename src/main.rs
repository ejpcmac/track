#![deny(unsafe_code)]
#![deny(warnings)]

use dirs::config_dir;
use std::fs;
use structopt::StructOpt;
use track::Client;

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
struct Opts {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    let api_key_file = config_dir().unwrap().join("track").join("api_key");
    let api_key = fs::read_to_string(api_key_file)?;

    let client = Client::new(&api_key);
    client.track(&opts.tracking_number)?;

    Ok(())
}
