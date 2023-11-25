/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod athena_error;
pub mod query_execution_state;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::query_execution_status::athena_error::AthenaError;
use crate::handler::query_output::query_execution::query_execution_status::query_execution_state::QueryExecutionState;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QueryExecutionStatus {
    pub state: Option<QueryExecutionState>,
    pub state_change_reason: Option<String>,
    pub submission_date_time: Option<i64>,
    pub completion_date_time: Option<i64>,
    pub athena_error: Option<AthenaError>,
}
