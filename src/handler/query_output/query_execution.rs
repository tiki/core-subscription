/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod result_configuration;
pub mod result_reuse_configuration;
pub mod query_execution_status;
pub mod query_execution_statistics;
pub mod statement_type;
pub mod query_execution_context;
pub mod engine_version;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::engine_version::EngineVersion;
use crate::handler::query_output::query_execution::query_execution_context::QueryExecutionContext;
use crate::handler::query_output::query_execution::query_execution_statistics::QueryExecutionStatistics;
use crate::handler::query_output::query_execution::query_execution_status::QueryExecutionStatus;
use crate::handler::query_output::query_execution::result_configuration::ResultConfiguration;
use crate::handler::query_output::query_execution::result_reuse_configuration::ResultReuseConfiguration;
use crate::handler::query_output::query_execution::statement_type::StatementType;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QueryExecution {
    pub query_execution_id: Option<String>,
    pub query: Option<String>,
    pub statement_type: Option<StatementType>,
    pub result_configuration: Option<ResultConfiguration>,
    pub result_reuse_configuration: Option<ResultReuseConfiguration>,
    pub query_execution_context: Option<QueryExecutionContext>,
    pub status: Option<QueryExecutionStatus>,
    pub statistics: Option<QueryExecutionStatistics>,
    pub work_group: Option<String>,
    pub engine_version: Option<EngineVersion>,
    pub execution_parameters: Option<Vec<String>>,
    pub substatement_type: Option<String>,
}
