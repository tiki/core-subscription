/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum QueryExecutionState {
    Cancelled,
    Failed,
    Queued,
    Running,
    #[default]
    Succeeded,
}
