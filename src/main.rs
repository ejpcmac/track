#![deny(unsafe_code)]
#![deny(warnings)]

use clap::{App, Arg};
use reqwest::{blocking::Client, header};
use serde::Deserialize;
use std::fs;

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
    date: String,
    label: String,
}

const API_ENDPOINT: &str = "https://api.laposte.fr/suivi/v2/idships/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("track")
        .version("0.1.0-dev")
        .author("Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")
        .about("A quick-and-dirty CLI tool for tracking parcels")
        .arg(Arg::with_name("tracking_number").required(true))
        .get_matches();

    let tracking_number = matches.value_of("tracking_number").unwrap();
    let url = API_ENDPOINT.to_owned() + tracking_number;

    let api_key = fs::read_to_string("api_key")?;

    let client = Client::new();
    let tracking_info: TrackingInfo = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .header("X-Okapi-Key", api_key)
        .send()?
        .json()?;

    for event in tracking_info.shipment.event.iter().rev() {
        println!("{}: {}", event.date, event.label);
    }

    Ok(())
}
