// track - A quick-and-dirty CLI tool for tracking parcels.
// Copyright (C) 2020 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! A quick-and-dirty client for the La Poste “Suivi v2” API.

use chrono::{DateTime, Local};
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
use std::{fs, io};

/// The client configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
}

/// An API client.
#[derive(Debug)]
pub struct Client {
    reqwest_client: reqwest::blocking::Client,
}

#[derive(Debug, Deserialize)]
struct TrackingInfo {
    shipment: Shipment,
}

#[derive(Debug, Deserialize)]
struct Shipment {
    event: Vec<Event>,
}

/// An event.
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The timestamp of the event.
    pub date: DateTime<Local>,

    /// A description of the event.
    pub label: String,
}

const API_ENDPOINT: &str = "https://api.laposte.fr/suivi/v2/idships/";

impl Config {
    /// Creates a new configuration.
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
        }
    }

    /// Loads the configuration.
    pub fn load() -> io::Result<Self> {
        let config_file = dirs::config_dir()
            .unwrap()
            .join("track")
            .join("config.toml");
        let contents = fs::read_to_string(config_file)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Saves the configuration.
    pub fn save(&self) -> io::Result<()> {
        let config_dir = dirs::config_dir().unwrap().join("track");
        fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.toml");
        let config = toml::to_string(self).unwrap();
        fs::write(config_file, config)?;

        Ok(())
    }
}

impl Client {
    /// Creates a new `Client`.
    pub fn new(config: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers.insert("X-Okapi-Key", config.api_key.parse().unwrap());

        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { reqwest_client }
    }

    /// Retrieves the events for a parcel.
    pub fn get_events(
        &self,
        tracking_number: &str,
    ) -> Result<Vec<Event>, reqwest::Error> {
        let url = API_ENDPOINT.to_owned() + tracking_number;

        let tracking_info: TrackingInfo =
            self.reqwest_client.get(&url).send()?.json()?;

        Ok(tracking_info.shipment.event)
    }
}
