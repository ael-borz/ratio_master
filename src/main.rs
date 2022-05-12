
/* TODO :
* Read .torrent file and extract relevant data (tracker URL, hash, size, nb of seeders/leechers)
* Read config file if exists
* if there is not config file: get user input
*  -> upload & download speed
*  -> range to randomize speed
* generate Client key, peer-id, port
* Simulate 30mn of seeding/leeching
* Send simulated data to tracker URL after gathering 30mn of data OR on user manual request
*/

use core::time;
use std::{collections::HashMap, hash::Hash, thread, time::Duration, ops::Add};

use clap::StructOpt;
use lava_torrent::torrent::v1::Torrent;
use ratiomaster::{cli_parser::Args, network::{update_tracker, set_initial_params, build_url, update_params}, engine::{FakeClient}};

const NUMBER_OF_PARAMS: usize = 10;
const UPDATE_RATE: u64 = 5; // Seconds

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let torrent = Torrent::read_from_file(args.torrent_file_path)?;
   
    let mut params: HashMap<&'static str, String> = HashMap::with_capacity(NUMBER_OF_PARAMS);
    set_initial_params(&mut params, &torrent);

    let base_url = torrent.announce.expect("Tracker URL not found");

    let mut fake_client = FakeClient::default();
    let mut elapsed_seconds = 0;
    loop {
        thread::sleep(time::Duration::from_secs(1));
        elapsed_seconds += 1;
        fake_client.seed_and_leech(10, 20);
        println!("downloaded: {}kB | uploaded: {}kB", fake_client.downloaded, fake_client.uploaded);
        if elapsed_seconds == UPDATE_RATE {
            update_params(&mut params, &fake_client);
            let url = build_url(&base_url, &params).expect("Failed to build url with params");
            println!("Sending request : {}", url);
            // update_tracker(url);
            elapsed_seconds = 0;
        }
    }
}