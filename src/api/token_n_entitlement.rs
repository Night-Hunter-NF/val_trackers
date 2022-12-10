use std::sync::Arc;

use reqwest::Client;
use tokio::sync::Mutex;

use crate::LockFile;

#[derive(Clone, Debug)]
pub struct TokenNEntitlement {
    token: Option<String>,
    client: Client,
    file: Arc<Mutex<Option<LockFile>>>,
}

impl TokenNEntitlement {
    pub fn new(client: Client, file: Arc<Mutex<Option<LockFile>>>) -> Self {
        Self {
            token: None,
            client,
            file,
        }
    }

    pub async fn get_token_n_entitlement(&mut self, file: LockFile) -> Result<String, TokenError> {
        if self.token.is_none() {
            let file = { self.file.lock().await.clone() };
            if let Some(file) = file {
                let token = self
                    .client
                    .get(format!(
                        "https://127.0.0.1:{}/entitlements/v1/token",
                        file.port
                    ))
                    .basic_auth("Authorization", Some(file.password))
                    .send()
                    .await
                    .map_err(TokenError::RequestError)?
                    .text()
                    .await
                    .map_err(TokenError::RequestError)?;
                self.token = Some(token.clone());
                return Ok(token);
            } else {
                return Err(TokenError::LockFile);
            }
        }
        Ok(self.token.clone().unwrap().clone())
    }
}

#[derive(Debug)]
pub enum TokenError {
    RequestError(reqwest::Error),
    LockFile,
}
// pub async fn get_token_n_entitlement(client: &Client) -> Result<Match, reqwest::Error> {
//     Ok(client
//         .get(format!("{}/matches/{}", BASE_URL, match_uuid))
//         .query(&[("source", "overwolf")])
//         .header("TRN-API-Key", TRN_API_KEY)
//         .send()
//         .await?
//         .json::<Root>()
//         .await?
//         .data)
// }
