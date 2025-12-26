// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::PostType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id:         Uuid,
    pub author_id:  Uuid,
    pub church_id:  Option<Uuid>,
    pub post_type:  PostType,
    pub title:      String,
    pub content:    String,
    pub media_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePost {
    pub church_id: Option<Uuid>,
    pub post_type: PostType,

    #[validate(length(min = 1, max = 300))]
    pub title: String,

    #[validate(length(min = 1, max = 50000))]
    pub content: String,

    #[validate(length(max = 10))]
    pub media_urls: Vec<String>
}
