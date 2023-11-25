/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod s3_acl_option;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::result_configuration::acl_configuration::s3_acl_option::S3AclOption;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AclConfiguration {
    pub s3_acl_option: S3AclOption
}
