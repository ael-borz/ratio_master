use crate::engine::TorrentData;

/// TODO : Send update to tracker
/// Handle response status
pub async fn update_tracker(tracker_address: &str, data: &TorrentData) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_url(tracker_address, data);
    let resp = reqwest::get(url).await?;
    println!("{:?}", resp.status());
    Ok(())
}

/// TODO : 
fn build_url(tracker_address: &str, data: &TorrentData) -> String {
    todo!()
}