#![deny(unsafe_code)]
#![deny(warnings)]

use chrono::{DateTime, Local};
use dirs::config_dir;
use reqwest::header::{self, HeaderMap};
use serde::Deserialize;
use std::{fs, io};

/// The client configuration.
#[derive(Debug)]
pub struct Config {
    api_key: String,
}

/// A tracking API client.
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

#[derive(Debug, Deserialize)]
struct Event {
    date: DateTime<Local>,
    label: String,
}

const API_ENDPOINT: &str = "https://api.laposte.fr/suivi/v2/idships/";

impl Config {
    /// Loads the configuration.
    pub fn load() -> io::Result<Self> {
        let api_key_file = config_dir().unwrap().join("track").join("api_key");
        let api_key = fs::read_to_string(api_key_file)?;

        Ok(Self { api_key })
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

    /// Retreives and prints the tracking info for a parcel.
    pub fn track(&self, tracking_number: &str) -> Result<(), reqwest::Error> {
        let url = API_ENDPOINT.to_owned() + tracking_number;

        let tracking_info: TrackingInfo =
            self.reqwest_client.get(&url).send()?.json()?;

        for event in tracking_info.shipment.event.iter().rev() {
            println!("{}: {}", event.date.to_rfc2822(), event.label);
        }

        Ok(())
    }
}
