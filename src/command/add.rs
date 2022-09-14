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

use crate::{state::State, success};

/// Arguments for `track add`.
#[derive(Debug, Parser)]
pub struct Add {
    /// The tracking number.
    tracking_number: String,
    /// A description for the parcel.
    description: String,
}

impl super::Command for Add {
    fn run(&self) -> Result<()> {
        let Self {
            tracking_number,
            description,
        } = self;

        let mut state = State::load()?;
        let added = state.add_parcel(tracking_number, description);
        state.save()?;

        match added {
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
