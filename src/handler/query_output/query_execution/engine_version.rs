/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EngineVersion {
    pub selected_engine_version: Option<String>,
    pub effective_engine_version: Option<String>,
}
