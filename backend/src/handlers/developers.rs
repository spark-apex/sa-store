/// 开发者认证 API
/// POST /developers/individual  — 个人开发者注册
/// POST /developers/enterprise  — 企业开发者注册
/// GET  /developers/:id         — 查看认证状态
/// GET  /developers/me          — 查看当前用户认证信息
use axum::{
    Router, Json,
    extract::{Path, State},
    routing::{get, post},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;
use crate::models::developer::{
    Developer, IndividualRegisterRequest, EnterpriseRegisterRequest,
};

/// 挂载开发者相关路由
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/developers/individual", post(register_individual))
        .route("/developers/enterprise", post(register_enterprise))
        .route("/developers/{id}", get(get_developer))
        .with_state(state)
}

/// 个人开发者注册
async fn register_individual(
    State(state): State<Arc<AppState>>,
    Json(req): Json<IndividualRegisterRequest>,
) -> Result<(StatusCode, Json<Developer>), StatusCode> {
    // 校验身份证号格式（18位）
    if req.id_card_number.len() != 18 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let dev = sqlx::query_as::<_, Developer>(
        r#"INSERT INTO developers (
            user_id, type, status,
            contact_name, contact_phone, contact_email,
            id_card_name, id_card_number, id_card_front_url, id_card_back_url
        ) VALUES ($1, 'individual', 'pending', $2, $3, $4, $5, $6, $7, $8)
        RETURNING *"#
    )
    .bind(Uuid::nil()) // 后续接入 JWT 获取真实 user_id
    .bind(&req.contact_name)
    .bind(&req.contact_phone)
    .bind(&req.contact_email)
    .bind(&req.id_card_name)
    .bind(&req.id_card_number)
    .bind(&req.id_card_front_url)
    .bind(&req.id_card_back_url)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok((StatusCode::CREATED, Json(dev)))
}

/// 企业开发者注册
async fn register_enterprise(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EnterpriseRegisterRequest>,
) -> Result<(StatusCode, Json<Developer>), StatusCode> {
    // 校验统一社会信用代码（18位）
    if req.credit_code.len() != 18 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let dev = sqlx::query_as::<_, Developer>(
        r#"INSERT INTO developers (
            user_id, type, status,
            contact_name, contact_phone, contact_email,
            company_name, credit_code, business_license_url,
            legal_person_name, legal_person_id,
            legal_person_id_front_url, legal_person_id_back_url,
            authorization_url, icp_license
        ) VALUES ($1, 'enterprise', 'pending', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *"#
    )
    .bind(Uuid::nil()) // 后续接入 JWT
    .bind(&req.contact_name)
    .bind(&req.contact_phone)
    .bind(&req.contact_email)
    .bind(&req.company_name)
    .bind(&req.credit_code)
    .bind(&req.business_license_url)
    .bind(&req.legal_person_name)
    .bind(&req.legal_person_id)
    .bind(&req.legal_person_id_front_url)
    .bind(&req.legal_person_id_back_url)
    .bind(&req.authorization_url)
    .bind(&req.icp_license)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok((StatusCode::CREATED, Json(dev)))
}

/// 查看开发者认证状态
async fn get_developer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Developer>, StatusCode> {
    sqlx::query_as::<_, Developer>("SELECT * FROM developers WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
