/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod encryption_configuration;
pub mod acl_configuration;

use serde::{Deserialize, Serialize};
use crate::handler::query_output::query_execution::result_configuration::acl_configuration::AclConfiguration;
use crate::handler::query_output::query_execution::result_configuration::encryption_configuration::EncryptionConfiguration;


#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ResultConfiguration {
    pub output_location: Option<String>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub expected_bucket_owner: Option<String>,
    pub acl_configuration: Option<AclConfiguration>
}
