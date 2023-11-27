/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::env;
use reqwest::Client;
use serde::{Deserialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretManager {
    #[serde(rename = "ARN")]
    pub arn: Option<String>,
    pub created_date: Option<String>,
    pub name: Option<String>,
    pub secret_binary: Option<String>,
    pub secret_string: Option<String>,
    pub result_metadata: Map<String, String>
}

impl SecretManager {
    pub async fn load(client: &Client) -> Self {
        let aws_session_token = match env::var("AWS_SESSION_TOKEN") {
            Ok(token) => token,
            Err(_) => panic!("Missing: AWS_SESSION_TOKEN"),
        };
        let secret_arn = match env::var("SECRET_ARN") {
            Ok(token) => token,
            Err(_) => panic!("Missing: SECRET_ARN"),
        };
        client
            .get(format!("http://localhost:2773/secretsmanager/get?secretId={}", secret_arn))
            .header("X-Aws-Parameters-Secrets-Token", aws_session_token)
            .send()
            .await
            .expect("Failed to retrieve required secrets")
            .json()
            .await
            .expect("Failed to deserialize required secrets")
    }
}
