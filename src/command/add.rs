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
pub struct Add {
    /// The tracking number.
    tracking_number: String,
    /// A description for the parcel.
    description: String,
}

impl super::Command for Add {
    fn run(&self) -> Result<()> {
        let state = State::load()?;
        state
            .add_parcel(&self.tracking_number, &self.description)
            .save()?;

        let message = match state.parcels().get(&self.tracking_number) {
            None => format!(
                "{} ({}) is now tracked.",
                self.description, self.tracking_number
            ),
            Some(old_description) => format!(
                "{} ({}) has been renamed to “{}”.",
                old_description, self.tracking_number, self.description
            ),
        };

        println!("{}", message.green().bold());

        Ok(())
    }
}
