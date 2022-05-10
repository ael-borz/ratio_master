use std::path::PathBuf;

use clap::Parser;

/// Ratio Master
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to .torrent file
    #[clap(short, long)]
    pub torrent_file_path: PathBuf,
}