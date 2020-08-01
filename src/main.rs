#![deny(unsafe_code)]
#![deny(warnings)]

use chrono::{DateTime, Local};
use dirs::config_dir;
use reqwest::{blocking::Client, header};
use serde::Deserialize;
use std::fs;
use structopt::StructOpt;

/// A quick-and-dirty CLI tool for tracking parcels
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
struct Opts {
    /// The tracking number
    #[structopt(name = "tracking_number")]
    tracking_number: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    let url = API_ENDPOINT.to_owned() + &opts.tracking_number;

    let api_key_file = config_dir().unwrap().join("track").join("api_key");
    let api_key = fs::read_to_string(api_key_file)?;

    let client = Client::new();
    let tracking_info: TrackingInfo = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .header("X-Okapi-Key", api_key)
        .send()?
        .json()?;

    for event in tracking_info.shipment.event.iter().rev() {
        println!("{}: {}", event.date.to_rfc2822(), event.label);
    }

    Ok(())
}
