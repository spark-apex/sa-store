/// API 路由 — 应用商店
pub mod apps;
pub mod update;
pub mod developers;

use axum::Router;
use std::sync::Arc;
use crate::AppState;

/// 挂载 /store 下所有路由
pub fn store_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(apps::routes(state.clone()))
        .merge(update::routes(state.clone()))
        .merge(developers::routes(state))
}
