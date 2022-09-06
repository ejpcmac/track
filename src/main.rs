// track - A quick-and-dirty CLI tool for tracking parcels.
// Copyright (C) 2020 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! A quick-and-dirty CLI tool for tracking parcels.

#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use colored::Colorize;
use std::io::{self, Write};
use structopt::StructOpt;
use track::client::{Client, Config, Event};
use track::State;

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
enum Command {
    /// Initialises the configuration
    Init(Init),

    /// Retrieves and prints tracking info for a parcel
    Info(Info),

    /// Prints the set of tracked parcels
    List,

    /// Adds a parcel to the tracked set
    Add(Add),

    /// Removes a parcel from the tracked set
    Remove(Remove),

    /// Retrieves and prints tracking info for all tracked parcels
    All,
}

#[derive(Debug, StructOpt)]
struct Init {
    /// Force the init process
    #[structopt(short, long)]
    force: bool,
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
        Command::Init(opts) => {
            if !opts.force && Config::load().is_ok() {
                println!(
                    "{}\n{}",
                    "There is already a configuration.".red().bold(),
                    "You can force the command by running `track init -f`."
                        .blue()
                );
                std::process::exit(1);
            }

            let mut input = String::new();

            print!("{}", "Enter your La Poste API key: ".bold());
            io::stdout().flush()?;
            io::stdin().read_line(&mut input)?;

            let config = Config::new(input.trim());
            config.save()?;

            println!(
                "{}",
                "The configuration has been initialised.".green().bold()
            );
        }

        Command::Info(opts) => match Config::load() {
            Ok(config) => {
                let client = Client::new(config);
                let events = client.get_events(&opts.tracking_number)?;
                print_events(&events);
            }
            Err(_) => no_config_message(),
        },

        Command::List => {
            let state = State::load()?;

            println!("\n{}\n", "--- Tracked parcels ---".bold());
            for (tracking_number, description) in state.parcels() {
                println!("{}: {}", tracking_number, description);
            }
            println!();
        }

        Command::Add(opts) => {
            let state = State::load()?;
            state
                .add_parcel(&opts.tracking_number, &opts.description)
                .save()?;

            let message = match state.parcels().get(&opts.tracking_number) {
                None => format!(
                    "{} ({}) is now tracked.",
                    opts.description, opts.tracking_number
                ),
                Some(old_description) => format!(
                    "{} ({}) has been renamed to “{}”.",
                    old_description, opts.tracking_number, opts.description
                ),
            };

            println!("{}", message.green().bold());
        }

        Command::Remove(opts) => {
            let state = State::load()?;

            let message = match state.parcels().get(&opts.tracking_number) {
                Some(description) => {
                    state.remove_parcel(&opts.tracking_number).save()?;
                    format!(
                        "{} ({}) is not tracked anymore.",
                        description, opts.tracking_number
                    )
                }
                None => format!("{} was not tracked.", opts.tracking_number),
            };

            println!("{}", message.green().bold());
        }

        Command::All => match Config::load() {
            Ok(config) => {
                let state = State::load()?;
                let client = Client::new(config);

                for (tracking_number, description) in state.parcels() {
                    let message = format!(
                        "\n--- {} ({}) ---\n",
                        description, tracking_number
                    );

                    println!("{}", message.bold());
                    let events = client.get_events(tracking_number)?;
                    print_events(&events);
                    println!();
                }
            }
            Err(_) => no_config_message(),
        },
    }

    Ok(())
}

fn no_config_message() {
    println!(
        "{}\n{}",
        "The configuration is absent or invalid.".red().bold(),
        "You can create a configuration by running `track init`.".blue()
    );
}

fn print_events(events: &[Event]) {
    for event in events.iter().rev() {
        let date = format!("{}:", event.date.to_rfc2822());
        println!("{} {}", date.bright_black(), event.label);
    }
}
