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
use lava_torrent::torrent::v1::Torrent;
use ratiomaster::{cli_parser::Args, engine::FakeClient, utils::percent_encode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let torrent = Torrent::read_from_file(args.torrent_file_path)?;
    let update_date = args.update_rate.unwrap_or(60); // 1mn
    let seed_rate = args.seed_rate.unwrap_or(1000) * 1024;
    let leech_rate = args.leech_rate.unwrap_or(0) * 1024;

    let hash_bytes = torrent.info_hash_bytes();
    println!("{:?} => {}", hash_bytes, percent_encode(&hash_bytes));

    let mut fake_client = FakeClient::new(&torrent);
    println!("{:?}", fake_client.params);
    fake_client.run(update_date, seed_rate, leech_rate).await?;

    Ok(())
}
