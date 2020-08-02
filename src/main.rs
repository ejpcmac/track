#![deny(unsafe_code)]
#![deny(warnings)]

use std::io::{self, Write};
use structopt::StructOpt;
use track::client::{Client, Config};
use track::State;

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
enum Command {
    /// Initialises the configuration
    Init,

    /// Retrives and prints tracking info for a parcel
    Info(Info),

    /// Prints the set of tracked parcels
    List,

    /// Adds a parcel to the tracked set
    Add(Add),

    /// Removes a parcel from the tracked set
    Remove(Remove),

    /// Retrives and prints tracking info for all tracked parcels
    All,
}

#[derive(Debug, StructOpt)]
struct Info {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,
}

#[derive(Debug, StructOpt)]
struct Add {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,

    /// A description for the parcel
    description: String,
}

#[derive(Debug, StructOpt)]
struct Remove {
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

        Command::List => {
            let state = State::load()?;

            for (tracking_number, description) in state.parcels() {
                println!("{}: {}", tracking_number, description);
            }
        }

        Command::Add(opts) => {
            let mut state = State::load()?;
            state.add_parcel(&opts.tracking_number, &opts.description);
            state.save()?;
        }

        Command::Remove(opts) => {
            let mut state = State::load()?;
            state.remove_parcel(&opts.tracking_number);
            state.save()?;
        }

        Command::All => {
            let state = State::load()?;
            let config = Config::load()?;
            let client = Client::new(config);

            for (tracking_number, description) in state.parcels() {
                println!("\n--- {} ({})\n", description, tracking_number);
                client.track(tracking_number)?;
                println!();
            }
        }
    }

    Ok(())
}
