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

//! A quick-and-dirty client for the La Poste “Suivi v2” API.

use chrono::{DateTime, Local};
use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
use serde::Deserialize;
use thiserror::Error;

/// A La Poste “Suivi v2” API client.
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

/// An error that can occur when creating a new `Client`.
#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("invalid API key")]
    InvalidApiKey(#[from] InvalidHeaderValue),
    #[error("impossible to create a client")]
    ClientBuilderError(#[from] reqwest::Error),
}

/// The API endpoint.
const API_ENDPOINT: &str = "https://api.laposte.fr/suivi/v2/idships/";

impl Client {
    /// Creates a new `Client`.
    pub fn new(api_key: &str) -> Result<Self, NewClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert("X-Okapi-Key", api_key.parse()?);

        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { reqwest_client })
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
