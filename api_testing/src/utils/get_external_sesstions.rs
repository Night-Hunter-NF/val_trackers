use std::collections::HashMap;

use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use super::get_lock_file::LockFile;

pub async fn get_external_sesstions(
    client: &Client,
    lockfile: &LockFile,
) -> Result<ExternalSesstions, reqwest::Error> {
    let resualt: HashMap<String, ExternalSesstions> = client
        .get(format!(
            "https://127.0.0.1:{}/product-session/v1/external-sessions",
            lockfile.port
        ))
        .basic_auth("riot", Some(lockfile.password.clone()))
        .send()
        .await?
        .json()
        .await?;
    Ok(resualt
        .into_values()
        .collect::<Vec<ExternalSesstions>>()
        .remove(0))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSesstions {
    pub exit_code: i64,
    pub exit_reason: Value,
    pub is_internal: bool,
    pub launch_configuration: LaunchConfiguration,
    pub patchline_full_name: String,
    pub patchline_id: String,
    pub phase: String,
    pub product_id: String,
    pub version: String,
}

impl ExternalSesstions {
    pub fn get_country(&self) -> String {
        self.launch_configuration.arguments[4]
            .split("=")
            .collect::<Vec<&str>>()[1].to_string()
    }
    pub fn get_puid(&self) -> String {
        self.launch_configuration.arguments[3]
            .split("=")
            .collect::<Vec<&str>>()[1].to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfiguration {
    pub arguments: Vec<String>,
    pub executable: String,
    pub locale: String,
    pub voice_locale: Value,
    pub working_directory: String,
}
