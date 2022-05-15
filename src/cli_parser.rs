use std::path::PathBuf;

use clap::Parser;

/// Ratio Master
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to .torrent file
    #[clap(short, long)]
    pub torrent_file_path: PathBuf,

    /// Update rate
    #[clap(short, long)]
    pub update_rate: Option<u64>,

    /// Seed rate
    #[clap(short, long)]
    pub seed_rate: Option<u32>,

    /// Leech rate
    #[clap(short, long)]
    pub leech_rate: Option<u32>,
}
