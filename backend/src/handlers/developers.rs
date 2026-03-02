/// 开发者接入 API（sa-store 维度）
/// 身份认证数据统一通过 sa-user 管理
/// sa-store 只负责：
///   1. 校验用户是否已在 sa-user 完成认证
///   2. 管理应用合规信息（隐私政策、权限、分级）
use axum::{
    Router,
    routing::get,
    http::StatusCode,
};
use std::sync::Arc;
use crate::AppState;

/// 挂载开发者相关路由
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        // 后续实现：
        // GET  /developers/verify-status  — 调 sa-user 检查当前用户认证状态
        // POST /apps/:key/compliance      — 提交应用合规信息
        // GET  /apps/:key/compliance      — 查询合规审核状态
        .with_state(state)
}
