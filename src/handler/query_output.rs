/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod query_execution;
pub mod execution;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::execution::Execution;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QueryOutput {
    pub query: Option<String>,
    pub request_id: Option<String>,
    pub execution: Option<Execution>,
}
