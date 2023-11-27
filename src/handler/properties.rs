/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod secret_manager;
mod secrets;

use std::env;
use reqwest::Client;
use crate::handler::properties::{secret_manager::SecretManager, secrets::Secrets};

#[derive(Debug, Clone)]
pub struct Properties {
    pub account_url: String,
    pub client_id: String,
    pub client_secret: String
}

impl Properties {
    pub async fn load(client: &Client) -> Self {
        let account_url = match env::var("ACCOUNT_URL") {
            Ok(token) => token,
            Err(_) => panic!("Missing: ACCOUNT_URL"),
        };
        let sm: SecretManager = SecretManager::load(client).await;
        let secrets: Secrets =  serde_json::from_str(
            &sm.secret_string.expect("Missing SecretString")
        ).expect("Failed to deserialize secrets");
        Properties {
            account_url,
            client_id: secrets.client_id.expect("Missing CLIENT_ID"),
            client_secret: secrets.client_secret.expect("MISSING CLIENT_SECRET"),
        }
    }
}
