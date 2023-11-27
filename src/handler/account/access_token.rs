/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use reqwest::Client;
use serde::{Deserialize};
use crate::handler::properties::Properties;

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>
}

impl AccessToken {
    pub async fn get(client: &Client, properties: &Properties) -> Result<Self, Box<dyn Error>> {
        let params = [
            ("grant_type", "client_credentials"),
            ("scope", "account:internal:ocean"),
            ("client_id", &format!("internal:{}",properties.client_id)),
            ("client_secret", &properties.client_secret)
        ];
        let response  = client
            .post(format!("{}/api/latest/auth/token", properties.account_url))
            .form(&params)
            .send()
            .await?
            .error_for_status()?;
        let token: AccessToken = response.json().await?;
        return Ok(token);
    }
}
