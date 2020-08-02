#![deny(unsafe_code)]
#![deny(warnings)]

pub mod client;

use im::HashMap;
use serde::{Deserialize, Serialize};
use std::{fs, io};

/// Tracking data.
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
