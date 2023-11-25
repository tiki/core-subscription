/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod encryption_option;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::result_configuration::encryption_configuration::encryption_option::EncryptionOption;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EncryptionConfiguration {
    pub encryption_option: Option<EncryptionOption>,
    pub kms_key: Option<String>
}
