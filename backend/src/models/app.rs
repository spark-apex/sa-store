/// 应用模型
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 应用记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct App {
    pub id: Uuid,
    pub app_key: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub screenshots: serde_json::Value,
    pub category: String,
    pub developer_id: Uuid,
    pub is_official: bool,
    pub status: String,
    pub total_downloads: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建应用请求
#[derive(Debug, Deserialize)]
pub struct CreateAppRequest {
    pub app_key: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub category: Option<String>,
    pub is_official: Option<bool>,
}

/// 应用列表响应（精简字段）
#[derive(Debug, Serialize)]
pub struct AppSummary {
    pub id: Uuid,
    pub app_key: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub category: String,
    pub is_official: bool,
    pub total_downloads: i64,
    pub latest_version: Option<String>,
}
