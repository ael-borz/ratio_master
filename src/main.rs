
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

use std::{collections::HashMap, hash::Hash};

use clap::StructOpt;
use lava_torrent::torrent::v1::Torrent;
use ratiomaster::{cli_parser::Args, network::{update_tracker, build_initial_params, build_url}};

const NUMBER_OF_PARAMS: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let torrent = Torrent::read_from_file(args.torrent_file_path)?;
   
    let mut params: HashMap<&'static str, String> = HashMap::with_capacity(NUMBER_OF_PARAMS);
    build_initial_params(&mut params, &torrent);
    println!("{:?}", params);

    // let base_url = torrent.announce.expect("Tracker URL not found");
    // let url = build_url(&base_url, &params).expect("Failed to build url with params");

    // update_tracker(url).await?;

    Ok(())
}