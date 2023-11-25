/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod result_reuse_by_age_configuration;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::result_reuse_configuration::result_reuse_by_age_configuration::ResultReuseByAgeConfiguration;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ResultReuseConfiguration {
    pub result_reuse_by_age_configuration: Option<ResultReuseByAgeConfiguration>,
}
