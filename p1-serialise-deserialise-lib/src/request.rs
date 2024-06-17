const HEADER: &str = "application/json";

use std::error::Error;

use crate::rust::RustSingleCycle;
use reqwest::{ self, blocking::Client, header::ACCEPT };

pub fn api_request_all_rust_cycles() -> Result<Vec<RustSingleCycle>, Box<dyn Error>> {
    let request_string: &str = "https://endoflife.date/api/rust.json";

    // Builds the client request
    let client = Client::new();

    // Our response body
    let body = client.get(request_string).header(ACCEPT, HEADER).send()?.bytes()?.to_vec();

    // Our response string
    let response = unsafe { String::from_utf8_unchecked(body) };
    let deserialised_response: Vec<RustSingleCycle> = serde_json::from_str(&response)?;
    Ok(deserialised_response)
}