use chrono::{DateTime, Local};
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
use std::{fs, io};

/// The client configuration.
#[derive(Debug, Serialize, Deserialize)]
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
