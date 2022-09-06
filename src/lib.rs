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

#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

pub mod client;

use std::{fs, io};

use im::HashMap;
use serde::{Deserialize, Serialize};

/// The persistent state for `track`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    parcels: Parcels,
}

/// A set of parcels.
type Parcels = HashMap<TrackingNumber, Description>;

/// A tracking number.
type TrackingNumber = String;

/// A parcel description.
type Description = String;

impl State {
    /// Creates empty tracking data.
    pub fn new() -> Self {
        Self {
            parcels: Parcels::new(),
        }
    }

    /// Loads the data.
    pub fn load() -> io::Result<Self> {
        let data_file =
            dirs::data_dir().unwrap().join("track").join("data.toml");

        match fs::read_to_string(data_file) {
            Ok(contents) => {
                let data = toml::from_str(&contents)?;
                Ok(data)
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(Self::new()),
                _ => Err(e),
            },
        }
    }

    /// Saves the data.
    pub fn save(&self) -> io::Result<()> {
        let data_dir = dirs::data_dir().unwrap().join("track");
        fs::create_dir_all(&data_dir)?;

        let data_file = data_dir.join("data.toml");
        let data = toml::to_string(self).unwrap();
        fs::write(data_file, data)?;

        Ok(())
    }

    /// Adds a parcel to track.
    pub fn add_parcel(&self, tracking_number: &str, description: &str) -> Self {
        Self {
            parcels: self
                .parcels
                .update(tracking_number.to_owned(), description.to_owned()),
        }
    }

    /// Removes a parcel.
    pub fn remove_parcel(&self, tracking_number: &str) -> Self {
        Self {
            parcels: self.parcels.without(tracking_number),
        }
    }

    /// Returns the set of tracked parcels.
    pub fn parcels(&self) -> &Parcels {
        &self.parcels
    }
}
