/// 开发者认证模型
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 开发者类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeveloperType {
    #[serde(rename = "individual")]
    Individual, // 个人
    #[serde(rename = "enterprise")]
    Enterprise, // 企业
}

/// 开发者档案
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Developer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String,
    pub status: String,

    // 通用信息
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,

    // 个人开发者
    pub id_card_name: Option<String>,
    pub id_card_number: Option<String>,
    pub id_card_front_url: Option<String>,
    pub id_card_back_url: Option<String>,

    // 企业开发者
    pub company_name: Option<String>,
    pub credit_code: Option<String>,
    pub business_license_url: Option<String>,
    pub legal_person_name: Option<String>,
    pub legal_person_id: Option<String>,
    pub legal_person_id_front_url: Option<String>,
    pub legal_person_id_back_url: Option<String>,
    pub authorization_url: Option<String>,

    // 附加资质
    pub icp_license: Option<String>,
    pub app_category_license_url: Option<String>,

    // 审核
    pub reject_reason: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub reviewer_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 个人开发者注册请求
#[derive(Debug, Deserialize)]
pub struct IndividualRegisterRequest {
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub id_card_name: String,
    pub id_card_number: String,
    pub id_card_front_url: String,
    pub id_card_back_url: String,
}

/// 企业开发者注册请求
#[derive(Debug, Deserialize)]
pub struct EnterpriseRegisterRequest {
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub company_name: String,
    pub credit_code: String,
    pub business_license_url: String,
    pub legal_person_name: String,
    pub legal_person_id: String,
    pub legal_person_id_front_url: String,
    pub legal_person_id_back_url: String,
    pub authorization_url: Option<String>,
    pub icp_license: Option<String>,
}

/// 开发者摘要（列表用）
#[derive(Debug, Serialize)]
pub struct DeveloperSummary {
    pub id: Uuid,
    pub r#type: String,
    pub status: String,
    pub contact_name: String,
    pub company_name: Option<String>,
    pub apps_count: i64,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

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
