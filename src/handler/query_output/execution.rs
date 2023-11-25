/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::QueryExecution;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Execution {
    pub query_execution: Option<QueryExecution>,
}
