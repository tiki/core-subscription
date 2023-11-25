/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EncryptionOption {
    CseKms,
    SseKms,
    #[default]
    SseS3,
}
