/* TODO :
* Read config file if exists
* Add range to randomize speed
*/

use clap::StructOpt;
use lava_torrent::torrent::v1::Torrent;
use ratiomaster::{cli_parser::Args, engine::FakeClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let torrent = Torrent::read_from_file(args.torrent_file_path)?;
    let update_date = args.update_rate.unwrap_or(60); // 1mn
    let seed_rate = args.seed_rate.unwrap_or(1000) * 1024;
    let leech_rate = args.leech_rate.unwrap_or(0) * 1024;

    let mut fake_client = FakeClient::new(&torrent);
    fake_client.run(update_date, seed_rate, leech_rate).await?;

    Ok(())
}
