/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod access_token;
mod account_result;

use std::error::Error;
use reqwest::Client;
use crate::handler::{
    account::access_token::AccessToken,
    account::account_result::AccountResult,
    properties::Properties,
    query_output::QueryOutput
};

pub async fn post_result(client: &Client, properties: &Properties, query_output: &QueryOutput) -> Result<(), Box<dyn Error>> {
    let token = AccessToken::get(client, properties).await?;
    let access_token = token.access_token.expect("Missing required access_token");
    let request_id = query_output.request_id.as_ref().expect("Missing required RequestId");
    let result_uri = query_output
        .execution.as_ref().expect("Missing required Execution")
        .query_execution.as_ref().expect("Missing required QueryExecution")
        .result_configuration.as_ref().expect("Missing required ResultConfiguration")
        .output_location.as_ref().expect("Missing required OutputLocation");
    AccountResult::new(&request_id, &result_uri)
        .send(client, &properties.account_url, &access_token)
        .await?
        .error_for_status()?;
    Ok(())
}
