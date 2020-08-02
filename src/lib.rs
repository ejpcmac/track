#![deny(unsafe_code)]
#![deny(warnings)]

use chrono::{DateTime, Local};
use reqwest::header;
use serde::Deserialize;

/// A tracking API client.
#[derive(Debug)]
pub struct Client {
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct TrackingInfo {
    shipment: Shipment,
}

#[derive(Debug, Deserialize)]
struct Shipment {
    event: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Event {
    date: DateTime<Local>,
    label: String,
}

const API_ENDPOINT: &str = "https://api.laposte.fr/suivi/v2/idships/";

impl Client {
    /// Creates a new `Client`.
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// Retreives and prints the tracking info for a parcel.
    pub fn track(&self, tracking_number: &str) -> Result<(), reqwest::Error> {
        let url = API_ENDPOINT.to_owned() + tracking_number;

        let reqwest_client = reqwest::blocking::Client::new();
        let tracking_info: TrackingInfo = reqwest_client
            .get(&url)
            .header(header::ACCEPT, "application/json")
            .header("X-Okapi-Key", &self.api_key)
            .send()?
            .json()?;

        for event in tracking_info.shipment.event.iter().rev() {
            println!("{}: {}", event.date.to_rfc2822(), event.label);
        }

        Ok(())
    }
}
