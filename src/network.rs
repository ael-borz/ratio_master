use std::collections::HashMap;

use lava_torrent::torrent::v1::Torrent;
use reqwest::Url;

use crate::engine::{generate_client_peer_id, generate_key, FakeClient};


/// TODO : Send update to tracker
/// Handle response status
pub async fn update_tracker(url: Url) -> Result<(), Box<dyn std::error::Error>> {    
    let resp = reqwest::get(url).await?;
    println!("{:?}", resp.status());
    Ok(())
}

pub fn set_initial_params(params: &mut HashMap<&'static str, String>, torrent: &Torrent) {
    params.insert("info_hash", torrent.info_hash());
    params.insert("peer_id", generate_client_peer_id("qbittorent"));
    params.insert("port", String::from("12828"));
    params.insert("uploaded", String::from("0"));
    params.insert("downloaded", String::from("0"));
    params.insert("left", torrent.length.to_string());
    params.insert("compact", String::from("1"));
    params.insert("numwant", String::from("200"));
    params.insert("event", String::from("started"));
    params.insert("key", generate_key());
}

pub fn update_params(params: &mut HashMap<&'static str, String>, fake_client: &FakeClient) {
    params.entry("downloaded").and_modify(|downloaded| *downloaded = fake_client.downloaded.to_string());
    params.entry("uploaded").and_modify(|uploaded| *uploaded = fake_client.uploaded.to_string());
}

pub fn build_url(url: &str, params: &HashMap<&'static str, String>) -> Option<Url> {
    reqwest::Url::parse_with_params(url, params).ok()
}