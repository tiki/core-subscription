/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use reqwest::{Client, Response};
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResult {
    pub request_id: String,
    pub result_uri: String
}

impl AccountResult {
    pub fn new(request_id: &str, result_uri: &str) -> Self {
        AccountResult {
            request_id: String::from(request_id),
            result_uri: String::from(result_uri)
        }
    }

    pub async fn send(self, client: &Client, account_url: &str, token: &str) -> Result<Response, Box<dyn Error>> {
        let res = client
            .post(format!("{}/api/latest/ocean", account_url))
            .bearer_auth(token)
            .json(&self)
            .send()
            .await?;
        Ok(res)
    }
}
