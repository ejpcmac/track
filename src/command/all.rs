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

use clap::Parser;
use color_eyre::Result;
use colored::Colorize;

use super::helpers::*;
use crate::{
    client::{Client, Config},
    state::State,
};

#[derive(Debug, Parser)]
pub struct All;

impl super::Command for All {
    fn run(&self) -> Result<()> {
        match Config::load() {
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
        }

        Ok(())
    }
}
