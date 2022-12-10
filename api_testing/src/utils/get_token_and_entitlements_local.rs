use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use super::get_lock_file::LockFile;

pub async fn get_token_and_entitlements_local(
    client: &Client,
    lockfile: &LockFile,
) -> Result<TokenAndEntitlements, reqwest::Error> {
    let resualt: TokenAndEntitlements = client
        .get(format!(
            "https://127.0.0.1:{}/entitlements/v1/token",
            lockfile.port
        ))
        .basic_auth("riot", Some(lockfile.password.clone()))
        .send()
        .await?
        .json()
        .await?;
    Ok(resualt)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAndEntitlements {
    pub access_token: String,
    pub entitlements: Vec<Value>,
    pub issuer: String,
    /// puuid
    pub subject: String,
    pub token: String,
}
