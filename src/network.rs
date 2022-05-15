use std::collections::HashMap;

use reqwest::Client;

pub fn build_client() -> Result<Client, reqwest::Error> {
    reqwest::Client::builder()
        .gzip(true)
        .user_agent("qBittorrent/4.4.2")
        .build()
}

pub fn build_url(url: &str, params: &HashMap<&'static str, String>) -> String {
    let key_values = params
        .iter()
        .map(|(k, v)| String::from(&format!("{}={}", k, v)))
        .collect::<Vec<String>>();
    let query_params = key_values.join("&");

    [url, &query_params].join("?")
}
