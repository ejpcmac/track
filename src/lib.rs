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
//!
//! # Rationale
//!
//! At the time of writing this tool, I am in some place with a really slow
//! internet access. This means refreshing the tracking information page on
//! [laposte.fr](https://www.laposte.fr/outils/suivre-vos-envois) takes minutes.
//! As I want to be able to track my parcels with a very low bandwidth impact, I
//! had the idea of using their public tracking API to do this from my terminal.
//!
//! In my browser, I used to keep some tabs open to track incoming parcels. With
//! `track`, I can now register which parcels to track and get an overview
//! pretty quickly. I am sure it will be helpful even with a high-speed internet
//! access.
//!
//! # Setup
//!
//! To use `track`, you need an account on [La Poste
//! Developer](https://developer.laposte.fr). You can then create a new
//! application—name it `track` for instance—and register to their [free
//! tracking API](https://developer.laposte.fr/products/suivi/latest) to get an
//! API key.
//!
//! Then, install `track`:
//!
//! ```shell
//! $ cargo install --git https://github.com/ejpcmac/track.git
//! ```
//!
//! Configure `track` to use your API key:
//!
//!     $ track init
//!
//! # Usage
//!
//! You can track an individual parcel:
//!
//!     $ track info <tracking_number>
//!
//! If you want to track a few parcels regularly, you can add them:
//!
//!     $ track add <tracking_number> <description>
//!
//! Then get their status:
//!
//!     $ track all
//!
//! You can list the tracked parcels:
//!
//!     $ track list
//!
//! Or simply remove one from the list:
//!
//!     $ track remove <tracking_number>
//!
//! # Caveats
//!
//! * The library crate API is unstable
//! * There is currently no proper error handling
//! * I have tested it only for Colissimo parcels

#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

pub mod client;

use im::HashMap;
use serde::{Deserialize, Serialize};
use std::{fs, io};

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
