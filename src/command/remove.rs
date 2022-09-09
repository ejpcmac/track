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

use crate::state::State;

#[derive(Debug, Parser)]
pub struct Remove {
    /// The tracking number.
    tracking_number: String,
}

impl super::Command for Remove {
    fn run(&self) -> Result<()> {
        let Self { tracking_number } = self;

        let state = State::load()?;

        let message = match state.parcels().get(tracking_number) {
            Some(description) => {
                state.remove_parcel(tracking_number).save()?;
                format!(
                    "{description} ({tracking_number}) is not tracked anymore."
                )
            }
            None => {
                format!("{tracking_number} was not tracked.")
            }
        };

        println!("{}", message.green().bold());

        Ok(())
    }
}
