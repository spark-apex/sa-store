/// 版本模型
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 版本记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppVersion {
    pub id: Uuid,
    pub app_id: Uuid,
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub signature: Option<String>,
    pub platform: String,
    pub status: String,
    pub review_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

/// 创建版本请求
#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    pub signature: Option<String>,
    pub platform: Option<String>,
}

/// Tauri updater 兼容的版本信息响应
#[derive(Debug, Serialize)]
pub struct TauriUpdateResponse {
    pub version: String,
    pub notes: String,
    pub pub_date: String,
    pub platforms: std::collections::HashMap<String, TauriPlatformInfo>,
}

/// Tauri 平台信息
#[derive(Debug, Serialize)]
pub struct TauriPlatformInfo {
    pub url: String,
    pub signature: String,
}
