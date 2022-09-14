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

use std::{collections::HashMap, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

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

/// An error that can occur when loading the state.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("impossible to locate the state file")]
    NoDataDir(#[from] DataDirError),
    #[error("error while reading the state file")]
    ReadError(#[from] io::Error),
    #[error("error while parsing the state file")]
    ParseError(#[from] toml::de::Error),
}

/// An error that can occur when saving the state.
#[derive(Debug, Error)]
pub enum SaveError {
    #[error("impossible to locate the state file")]
    NoDataDir(#[from] DataDirError),
    #[error("error while writing to the state file")]
    FsError(#[from] io::Error),
}

/// An error that can occur when getting the data directory.
#[derive(Debug, Error)]
pub enum DataDirError {
    #[error("the OS does not define a data directory")]
    NoDataDir,
}

/// The name of the state file.
const STATE_FILE_NAME: &str = "state.toml";

impl State {
    /// Creates empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the state.
    pub fn load() -> Result<Self, LoadError> {
        match fs::read_to_string(state_file()?) {
            Ok(state) => Ok(toml::from_str(&state)?),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(Self::new()),
                _ => Err(e.into()),
            },
        }
    }

    /// Saves the state.
    pub fn save(&self) -> Result<(), SaveError> {
        fs::create_dir_all(data_dir()?)?;

        let state =
            toml::to_string(self).expect("failed to serialise the state");
        fs::write(state_file()?, state)?;

        Ok(())
    }

    /// Adds a parcel to track.
    pub fn add_parcel(
        &mut self,
        tracking_number: &str,
        description: &str,
    ) -> Option<Description> {
        self.parcels
            .insert(tracking_number.to_owned(), description.to_owned())
    }

    /// Removes a parcel.
    pub fn remove_parcel(
        &mut self,
        tracking_number: &str,
    ) -> Option<Description> {
        self.parcels.remove(tracking_number)
    }

    /// Returns the set of tracked parcels.
    pub fn parcels(&self) -> &Parcels {
        &self.parcels
    }
}

/// Gets the data directory for `track`.
fn data_dir() -> Result<PathBuf, DataDirError> {
    Ok(dirs::data_dir()
        .ok_or(DataDirError::NoDataDir)?
        .join(env!("CARGO_PKG_NAME")))
}

/// Gets the state file for `track`.
fn state_file() -> Result<PathBuf, DataDirError> {
    Ok(data_dir()?.join(STATE_FILE_NAME))
}
