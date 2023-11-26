/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod query_output;
mod properties;

use lambda_runtime::{Error, LambdaEvent};
use crate::handler::{properties::Properties, query_output::QueryOutput};

pub async fn handle(event: LambdaEvent<QueryOutput>) -> Result<(), Error> {
    tracing::debug!("{:?}", event.payload);
    let client = reqwest::Client::new();
    let properties: Properties = Properties::load(&client).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn placeholder() {
        assert_eq!(1,1)
    }
}
