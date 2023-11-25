/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AthenaError {
    pub error_category: Option<i32>,
    pub error_type: Option<i32>,
    pub retryable: bool,
    pub error_message: Option<String>,
}
