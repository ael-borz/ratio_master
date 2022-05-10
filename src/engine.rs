use std::fmt::Display;

/// TODO : Better suited types
pub struct TorrentData {
    hash: String,
    peer_id: Vec<u8>,
    port: u16,
    downloaded: u32,
    uploaded: u32,
    left: u32,
    compact: bool,
    numwant: u8,
    event: Event,
    key: String,
}

impl TorrentData {
    pub fn new(hash: String, size: u32) -> Self {
        let peer_id: Vec<u8> = vec![0]; // TODO : Generate random
        let port: u16 = 12828; // TODO : Generate random
        let numwant: u8 = 200; // TODO : User input ?
        let key = String::from("34A687C0"); // TODO : Generate random hex ?
        
        TorrentData {
            hash,
            peer_id,
            port,
            downloaded: 0,
            uploaded: 0,
            left: size,
            compact: true,
            numwant,
            event: Event::Started,
            key,
        }
    }
}

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
