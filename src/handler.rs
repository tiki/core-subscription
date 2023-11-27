/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod query_output;
mod properties;
mod account;

use lambda_runtime::{Error, LambdaEvent};
use crate::handler::{properties::Properties, query_output::QueryOutput};

pub async fn handle(event: LambdaEvent<QueryOutput>) -> Result<(), Error> {
    tracing::debug!("{:?}", event.payload);
    let client = reqwest::Client::new();
    let properties: Properties = Properties::load(&client).await;
    let res = account::post_result(&client, &properties, &event.payload).await;
    match res {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!(?err, "Event handling failed.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn placeholder() {
        assert_eq!(1,1)
    }
}
