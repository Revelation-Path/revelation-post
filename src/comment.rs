// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostComment {
    pub id:         Uuid,
    pub post_id:    Uuid,
    pub author_id:  Uuid,
    pub content:    String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1, max = 5000))]
    pub content: String
}
