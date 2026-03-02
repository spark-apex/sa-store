/// 应用合规信息模型（商店维度）
/// 开发者身份认证数据统一存储在 sa-user 中
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 应用合规信息
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppCompliance {
    pub id: Uuid,
    pub app_id: Uuid,
    pub privacy_policy_url: Option<String>,
    pub user_agreement_url: Option<String>,
    pub permissions_desc: Option<String>,
    pub data_collection: serde_json::Value,
    pub age_rating: String,
    pub has_ads: bool,
    pub has_iap: bool,
    pub review_status: String,
    pub review_note: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewer_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 提交合规信息请求
#[derive(Debug, Deserialize)]
pub struct ComplianceRequest {
    pub privacy_policy_url: String,
    pub user_agreement_url: Option<String>,
    pub permissions_desc: Option<String>,
    pub data_collection: Option<serde_json::Value>,
    pub age_rating: Option<String>,
    pub has_ads: Option<bool>,
    pub has_iap: Option<bool>,
}
