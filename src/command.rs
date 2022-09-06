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

use std::io::{self, Write};

use clap::Parser;
use color_eyre::Result;
use colored::Colorize;

use crate::{
    client::{Client, Config, Event},
    state::State,
};

/// A quick-and-dirty CLI tool for tracking parcels.
#[derive(Debug, Parser)]
#[clap(version, author)]
pub enum Track {
    /// Initialise the configuration.
    Init(Init),
    /// Retrieve and prints tracking info for a parcel.
    Info(Info),
    /// Print the set of tracked parcels.
    List,
    /// Add a parcel to the tracked set.
    Add(Add),
    /// Remove a parcel from the tracked set.
    Remove(Remove),
    /// Retrieve and prints tracking info for all tracked parcels.
    All,
}

#[derive(Debug, Parser)]
pub struct Init {
    /// Force the init process.
    #[clap(short, long)]
    force: bool,
}

#[derive(Debug, Parser)]
pub struct Info {
    /// The tracking number.
    tracking_number: String,
}

#[derive(Debug, Parser)]
pub struct Add {
    /// The tracking number.
    tracking_number: String,
    /// A description for the parcel.
    description: String,
}

#[derive(Debug, Parser)]
pub struct Remove {
    /// The tracking number.
    tracking_number: String,
}

impl Track {
    /// Runs track.
    pub fn run() -> Result<()> {
        match Self::parse() {
            Self::Init(opts) => {
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

            Self::Info(opts) => match Config::load() {
                Ok(config) => {
                    let client = Client::new(config);
                    let events = client.get_events(&opts.tracking_number)?;
                    print_events(&events);
                }
                Err(_) => no_config_message(),
            },

            Self::List => {
                let state = State::load()?;

                println!("\n{}\n", "--- Tracked parcels ---".bold());
                for (tracking_number, description) in state.parcels() {
                    println!("{}: {}", tracking_number, description);
                }
                println!();
            }

            Self::Add(opts) => {
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

            Self::Remove(opts) => {
                let state = State::load()?;

                let message = match state.parcels().get(&opts.tracking_number) {
                    Some(description) => {
                        state.remove_parcel(&opts.tracking_number).save()?;
                        format!(
                            "{} ({}) is not tracked anymore.",
                            description, opts.tracking_number
                        )
                    }
                    None => {
                        format!("{} was not tracked.", opts.tracking_number)
                    }
                };

                println!("{}", message.green().bold());
            }

            Self::All => match Config::load() {
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
