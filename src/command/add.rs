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
use eyre::Result;
use inquire::Text;

use crate::{state::State, success};

/// Arguments for `track add`.
#[derive(Debug, Parser)]
pub struct Add {
    /// The tracking number.
    tracking_number: Option<String>,
    /// A description for the parcel.
    description: Option<String>,
}

impl super::Command for Add {
    fn run(&self) -> Result<()> {
        let tracking_number = match self.tracking_number.to_owned() {
            Some(value) => value,
            None => ask_tracking_number()?,
        };

        let description = match self.description.to_owned() {
            Some(value) => value,
            None => ask_description()?,
        };

        let mut state = State::load()?;
        let old = state.add_parcel(&tracking_number, &description);
        state.save()?;

        match old {
            None => {
                success!("{description} ({tracking_number}) is now tracked.")
            }
            Some(old_description) => success!(
                "{old_description} ({tracking_number}) has been renamed to “{description}”."
            ),
        };

        Ok(())
    }
}

/// Asks for the tracking number.
fn ask_tracking_number() -> Result<String> {
    Ok(Text::new("Tracking number:").prompt()?)
}

/// Asks for a description.
fn ask_description() -> Result<String> {
    Ok(Text::new("Description:").prompt()?)
}
