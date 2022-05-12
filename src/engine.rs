use std::{fmt::Display, collections::HashMap};

use rand::{Rng, thread_rng, distributions::Alphanumeric};


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
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    match client_name {
        "qbittorent" => format!("{}{}", "-qB4420-", rand_string),
        _ => format!("{}{}", "-qB4420-", rand_string)
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

#[derive(Debug, Default)]
pub struct FakeClient {
    pub downloaded: u32,
    pub uploaded: u32
}

impl FakeClient {
    /// TODO : Add random bytes
    pub fn seed_and_leech(&mut self, seed_rate: u32, leech_rate: u32) {
        self.downloaded += leech_rate;
        self.uploaded += seed_rate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_client_peer_id_correct_len() {
        let generated_peer_id = generate_client_peer_id("qbittorent");
        assert_eq!(generated_peer_id.len(), 20);
    }

    #[test]
    fn test_generate_key_correct_len() {
        let generated_key = generate_key();
        assert_eq!(generated_key.len(), 8);
    }
}
