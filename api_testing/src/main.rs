pub mod utils;

use std::env;

use reqwest::Client;
use tokio::fs;

use crate::utils::{
    get_external_sesstions::get_external_sesstions, get_lock_file::get_lock_file,
    get_token_and_entitlements_local::get_token_and_entitlements_local,
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let lockfile = get_lock_file().await;
    let tokens = get_token_and_entitlements_local(&client, &lockfile).await?;

    // let country = get_country(&client, &lockfile).await?;

    // println!("resualt: {}", tokens.);

    Ok(())
}
