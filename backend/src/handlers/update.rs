/// 更新检查 API — Tauri updater 兼容
/// GET /apps/:app_key/latest   — 返回 Tauri 标准格式的最新版本信息
/// GET /apps/:app_key/download — 302 重定向到下载地址
use axum::{
    Router, Json,
    extract::{Path, State},
    routing::get,
    http::StatusCode,
    response::Redirect,
};
use std::collections::HashMap;
use std::sync::Arc;
use crate::AppState;
use crate::models::app::App;
use crate::models::version::{AppVersion, TauriUpdateResponse, TauriPlatformInfo};

/// 挂载更新相关路由
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/apps/{app_key}/latest", get(check_update))
        .route("/apps/{app_key}/download", get(download_latest))
        .with_state(state)
}

/// 检查更新 — 返回 Tauri updater 兼容 JSON
/// 客户端 tauri.conf.json 的 endpoints 指向此接口
async fn check_update(
    State(state): State<Arc<AppState>>,
    Path(app_key): Path<String>,
) -> Result<Json<TauriUpdateResponse>, StatusCode> {
    // 查应用
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_key = $1 AND status = 'published'")
        .bind(&app_key)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 查最新已审核版本
    let version = sqlx::query_as::<_, AppVersion>(
        "SELECT * FROM app_versions WHERE app_id = $1 AND status = 'approved' ORDER BY created_at DESC LIMIT 1"
    )
    .bind(app.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // 构建 Tauri 标准响应
    let mut platforms = HashMap::new();
    platforms.insert(version.platform.clone(), TauriPlatformInfo {
        url: version.download_url.clone(),
        signature: version.signature.clone().unwrap_or_default(),
    });

    Ok(Json(TauriUpdateResponse {
        version: version.version,
        notes: version.changelog.unwrap_or_default(),
        pub_date: version.created_at.to_rfc3339(),
        platforms,
    }))
}

/// 下载最新版 — 302 重定向到 OSS 地址 + 下载计数
async fn download_latest(
    State(state): State<Arc<AppState>>,
    Path(app_key): Path<String>,
) -> Result<Redirect, StatusCode> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_key = $1 AND status = 'published'")
        .bind(&app_key)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let version = sqlx::query_as::<_, AppVersion>(
        "SELECT * FROM app_versions WHERE app_id = $1 AND status = 'approved' ORDER BY created_at DESC LIMIT 1"
    )
    .bind(app.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // 更新下载计数
    let _ = sqlx::query("UPDATE apps SET total_downloads = total_downloads + 1 WHERE id = $1")
        .bind(app.id)
        .execute(&state.db)
        .await;

    // 记录下载日志
    let _ = sqlx::query(
        "INSERT INTO download_logs (app_id, version_id) VALUES ($1, $2)"
    )
    .bind(app.id)
    .bind(version.id)
    .execute(&state.db)
    .await;

    Ok(Redirect::temporary(&version.download_url))
}
