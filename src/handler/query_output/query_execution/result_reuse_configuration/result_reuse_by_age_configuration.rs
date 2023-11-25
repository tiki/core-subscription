/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ResultReuseByAgeConfiguration {
    pub enabled: bool,
    pub max_age_in_minutes: Option<i32>,
}
