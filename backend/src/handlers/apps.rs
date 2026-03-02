/// 应用管理 API
/// GET  /apps              — 应用列表
/// GET  /apps/:app_key     — 应用详情
/// POST /apps              — 注册应用（需认证）
/// POST /apps/:app_key/versions — 发布新版本（需认证）
use axum::{
    Router, Json,
    extract::{Path, Query, State},
    routing::get,
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;
use crate::models::app::{App, CreateAppRequest, AppSummary};
use crate::models::version::{AppVersion, CreateVersionRequest};

/// 列表查询参数
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub category: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

/// 挂载应用相关路由
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/apps", get(list_apps).post(create_app))
        .route("/apps/{app_key}", get(get_app))
        .route("/apps/{app_key}/versions", get(list_versions).post(create_version))
        .with_state(state)
}

/// 应用列表
async fn list_apps(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<AppSummary>>, StatusCode> {
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = (q.page.unwrap_or(1) - 1).max(0) * limit;

    let apps = sqlx::query_as::<_, App>(
        r#"SELECT * FROM apps WHERE status = 'published'
           ORDER BY total_downloads DESC, created_at DESC
           LIMIT $1 OFFSET $2"#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 查最新版本号
    let mut summaries = Vec::new();
    for app in apps {
        let latest: Option<String> = sqlx::query_scalar(
            "SELECT version FROM app_versions WHERE app_id = $1 AND status = 'approved' ORDER BY created_at DESC LIMIT 1"
        )
        .bind(app.id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

        summaries.push(AppSummary {
            id: app.id, app_key: app.app_key, name: app.name,
            description: app.description, logo_url: app.logo_url,
            category: app.category, is_official: app.is_official,
            total_downloads: app.total_downloads, latest_version: latest,
        });
    }
    Ok(Json(summaries))
}

/// 应用详情
async fn get_app(
    State(state): State<Arc<AppState>>,
    Path(app_key): Path<String>,
) -> Result<Json<App>, StatusCode> {
    sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_key = $1")
        .bind(&app_key)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// 注册应用（暂不做 JWT 验证，后续接入 sa-user）
async fn create_app(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAppRequest>,
) -> Result<(StatusCode, Json<App>), StatusCode> {
    let app = sqlx::query_as::<_, App>(
        r#"INSERT INTO apps (app_key, name, description, logo_url, category, developer_id, is_official, status)
           VALUES ($1, $2, $3, $4, $5, $6, $7, 'published')
           RETURNING *"#
    )
    .bind(&req.app_key)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.logo_url)
    .bind(req.category.as_deref().unwrap_or("tool"))
    .bind(Uuid::nil()) // 暂用空 UUID，后续接入 sa-user JWT
    .bind(req.is_official.unwrap_or(false))
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok((StatusCode::CREATED, Json(app)))
}

/// 版本列表
async fn list_versions(
    State(state): State<Arc<AppState>>,
    Path(app_key): Path<String>,
) -> Result<Json<Vec<AppVersion>>, StatusCode> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_key = $1")
        .bind(&app_key)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let versions = sqlx::query_as::<_, AppVersion>(
        "SELECT * FROM app_versions WHERE app_id = $1 ORDER BY created_at DESC"
    )
    .bind(app.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(versions))
}

/// 发布新版本
async fn create_version(
    State(state): State<Arc<AppState>>,
    Path(app_key): Path<String>,
    Json(req): Json<CreateVersionRequest>,
) -> Result<(StatusCode, Json<AppVersion>), StatusCode> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_key = $1")
        .bind(&app_key)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 官方应用自动通过审核
    let status = if app.is_official { "approved" } else { "pending" };

    let version = sqlx::query_as::<_, AppVersion>(
        r#"INSERT INTO app_versions (app_id, version, changelog, download_url, file_size, file_hash, signature, platform, status)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING *"#
    )
    .bind(app.id)
    .bind(&req.version)
    .bind(&req.changelog)
    .bind(&req.download_url)
    .bind(req.file_size.unwrap_or(0))
    .bind(&req.file_hash)
    .bind(&req.signature)
    .bind(req.platform.as_deref().unwrap_or("windows-x86_64"))
    .bind(status)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok((StatusCode::CREATED, Json(version)))
}
