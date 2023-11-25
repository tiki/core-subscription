/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod result_reuse_information;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::query_execution_statistics::result_reuse_information::ResultReuseInformation;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QueryExecutionStatistics {
    pub engine_execution_time_in_millis: Option<i64>,
    pub data_scanned_in_bytes: Option<i64>,
    pub data_manifest_location: Option<String>,
    pub total_execution_time_in_millis: Option<i64>,
    pub query_queue_time_in_millis: Option<i64>,
    pub service_pre_processing_time_in_millis: Option<i64>,
    pub query_planning_time_in_millis: Option<i64>,
    pub service_processing_time_in_millis: Option<i64>,
    pub result_reuse_information: Option<ResultReuseInformation>,
}
