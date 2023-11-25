/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod query_output;

use lambda_runtime::{Error, LambdaEvent};
use crate::handler::query_output::QueryOutput;

pub async fn handle(event: LambdaEvent<QueryOutput>) -> Result<(), Error> {
    tracing::debug!("{:?}", event.payload);
    Ok(())
}

#[cfg(test)]
mod tests {
    use lambda_runtime::{Context, LambdaEvent};
    use tokio_test::assert_ok;
    use crate::handler::{
        query_output::QueryOutput,
        query_output::query_execution::QueryExecution,
        handle
    };

    #[tokio::test]
    async fn local() {
        let event = QueryOutput {
            query: Some(String::from("dummy")),
            request_id: Some(String::from("1234")),
            execution: Some(QueryExecution::default())
        };
        let event: LambdaEvent<QueryOutput> = LambdaEvent::<QueryOutput> {
            payload: event,
            context: Context::default()
        };
        let res = handle(event).await;
        assert_ok!(res);
    }
}
