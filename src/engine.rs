use core::time;
use std::{collections::HashMap, fmt::Display, io::Write, thread};

use lava_torrent::torrent::v1::Torrent;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Client;

use crate::{
    network::{build_client, build_url},
    utils::percent_encode,
};

/// Generates a new peer_id specific to the desired Bittorent client (20 bytes)
///
/// Example : qBittorrent Peer ID is formatted as follows: -qBXYZ0-<12 random bytes>
/// Where:
///
/// * X is the major version number
/// * Y is the minor version number
/// * Z is the bugfix version number (in hexadecimal so that we can go up to 15)
///
/// For example, we would have the following Peer IDs for these versions:
///
/// * qBittorrent v2.4.10: -qB24A0-<12 random bytes>
/// * qBittorrent v3.0.2: -qB3020-<12 random bytes>
pub fn generate_client_peer_id(client_name: &str) -> String {
    let rand_bytes: Vec<u8> = thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(12)
        .collect();

    let url_encoded_rand_bytes = percent_encode(&rand_bytes);

    match client_name {
        "qbittorent" => format!("{}{}", "-qB4420-", url_encoded_rand_bytes),
        _ => format!("{}{}", "-qB4420-", url_encoded_rand_bytes),
    }
}

pub fn generate_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    rand_string
}

#[derive(Debug)]
enum Event {
    Started,
    Stopped,
    Completed,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Started => f.write_str("started"),
            Event::Stopped => f.write_str("stopped"),
            Event::Completed => f.write_str("completed"),
        }
    }
}

const NUMBER_OF_PARAMS: usize = 11;

#[derive(Debug)]
pub struct FakeClient {
    client: Client,
    base_url: String,
    downloaded: u32,
    uploaded: u32,
    left: u32,
    pub params: HashMap<&'static str, String>,
    is_completed: bool,
}

impl FakeClient {
    pub fn new(torrent: &Torrent) -> Self {
        let mut params: HashMap<&'static str, String> = HashMap::with_capacity(NUMBER_OF_PARAMS);
        params.insert("info_hash", percent_encode(&torrent.info_hash_bytes()));
        params.insert("peer_id", generate_client_peer_id("qbittorent"));
        params.insert("port", String::from("12828"));
        params.insert("uploaded", String::from("0"));
        params.insert("downloaded", String::from("0"));
        params.insert("left", torrent.length.to_string());
        params.insert("compact", String::from("1"));
        params.insert("numwant", String::from("200"));
        params.insert("event", Event::Started.to_string());
        params.insert("key", generate_key());
        params.insert("no_peer_id", String::from("1"));

        let base_url = torrent.announce.as_ref().expect("Tracker URL not found");

        Self {
            client: build_client().expect("Could not build client"),
            base_url: base_url.to_string(),
            downloaded: 0,
            uploaded: 0,
            left: torrent.length as u32,
            params,
            is_completed: false,
        }
    }

    fn update_params(&mut self, state: Option<Event>) {
        self.params
            .entry("downloaded")
            .and_modify(|downloaded| *downloaded = self.downloaded.to_string());
        self.params
            .entry("uploaded")
            .and_modify(|uploaded| *uploaded = self.uploaded.to_string());
        if let Some(state) = state {
            self.params
                .entry("event")
                .and_modify(|event| *event = state.to_string());
        } else {
            self.params.remove("event");
        }
    }

    /// Simulates seeding & leeching
    ///
    /// Rates are exprimed in KB/s
    pub fn seed_and_leech(&mut self, seed_rate: u32, leech_rate: u32) {
        self.downloaded += leech_rate;
        self.uploaded += seed_rate;
    }

    pub async fn run(
        &mut self,
        update_rate: u64,
        seed_rate: u32,
        leech_rate: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send_request().await?; // Handshake
        let mut elapsed_seconds = 0;
        loop {
            thread::sleep(time::Duration::from_secs(1));
            elapsed_seconds += 1;
            self.seed_and_leech(seed_rate, leech_rate);
            self.show_progression();
            if !self.is_completed && self.downloaded >= self.left {
                self.update_params(Some(Event::Completed));
                self.send_request().await?;
                self.is_completed = true;
                elapsed_seconds = 0;
            } else if elapsed_seconds == update_rate {
                self.update_params(None);
                self.send_request().await?;
                elapsed_seconds = 0;
            }
        }
    }

    fn show_progression(&self) {
        print!(
            "\rdownloaded: {}kB | uploaded: {}kB",
            self.downloaded, self.uploaded
        );
        std::io::stdout().flush().expect("Flush failed");
    }

    async fn send_request(&self) -> Result<(), Box<dyn std::error::Error>> {
        //let url = build_url(&self.base_url, &self.params).expect("Failed to build url with params");
        let url = build_url(&self.base_url, &self.params);
        println!("Sending request : {}", url);
        let resp = self.client.get(url).send().await?;
        if resp.status().is_success() {
            println!("Update success");
        } else {
            eprintln!("Update failed : {:?}", resp);
        }
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_params(Some(Event::Stopped));
        self.send_request().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::percent_decode;

    use super::*;

    #[test]
    fn test_generate_client_peer_id_correct_len() {
        let generated_peer_id = generate_client_peer_id("qbittorent");
        assert_eq!(percent_decode(generated_peer_id.as_bytes()).len(), 20);
    }

    #[test]
    fn test_generate_key_correct_len() {
        let generated_key = generate_key();
        assert_eq!(generated_key.len(), 8);
    }
}
