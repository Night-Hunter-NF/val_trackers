use types::profile::get_profile;

use crate::types::{matchs::get_matchs, single_match::get_match};


pub mod types;

const BASE_URL: &str = "https://public-api.tracker.gg/v2/valorant/standard"; 
const TRN_API_Key: &str = "319e5540-bd60-4f5a-9660-6858c9a01350"; 

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new().build().unwrap();

    let response = get_match(client, "de9f5617-81c3-41f9-bed5-b866872f8df4").await.unwrap();
    println!("{}", response.metadata.map_image_url);
}