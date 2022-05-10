
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

use clap::StructOpt;
use ratiomaster::{cli_parser::Args, network::update_tracker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:?}", args.torrent_file_path);
    Ok(())
}