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
use eyre::{eyre, Result};
use inquire::Select;
use regex::Regex;

use crate::{state::State, success};

/// Arguments for `track remove`.
#[derive(Debug, Parser)]
pub struct Remove {
    /// The tracking number.
    tracking_number: Option<String>,
}

impl super::Command for Remove {
    fn run(&self) -> Result<()> {
        let mut state = State::load()?;

        let tracking_number = match self.tracking_number.to_owned() {
            Some(value) => value,
            None => ask_parcel(&state)?,
        };

        let description = state
            .remove_parcel(&tracking_number)
            .ok_or_else(|| eyre!("{tracking_number} was not tracked"))?;

        state.save()?;

        success!("{description} ({tracking_number}) is not tracked anymore.");
        Ok(())
    }
}

/// Asks for the parcel to delete.
fn ask_parcel(state: &State) -> Result<String> {
    let parcels = state.parcels();

    if parcels.is_empty() {
        return Err(eyre!("there are no tracked parcels"));
    }

    let options = parcels.iter().map(to_option).collect();
    let selected = Select::new("Parcel to remove", options).prompt()?;
    let tracking_number = extract_tracking_number(&selected)?;

    Ok(tracking_number)
}

/// Builds an option from a parcel tuple.
fn to_option(parcel: (&String, &String)) -> String {
    let (tracking_number, description) = parcel;
    format!("{tracking_number}: {description}")
}

/// Extracts the tracking number from a selected option.
fn extract_tracking_number(option: &str) -> Result<String> {
    let pattern = Regex::new(r"^(\w+): .*$")?;
    let captures = pattern
        .captures(option)
        .expect("failed to extract the tracking number from the selection");
    let tracking_number = captures[1].to_owned();
    Ok(tracking_number)
}
