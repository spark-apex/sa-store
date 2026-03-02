/// SA-Store 应用商店 API 服务
/// 提供应用注册、版本管理、更新检查、下载统计
mod config;
mod handlers;
mod models;
mod middleware;

use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::sync::Arc;

/// 共享应用状态
pub struct AppState {
    pub db: PgPool,
    pub config: config::Config,
}

/// 启动服务
#[tokio::main]
pub async fn run() {
    // 加载 .env
    dotenvy::dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sa_store_backend=debug,tower_http=info".into()),
        )
        .init();

    // 加载配置
    let cfg = config::Config::from_env();

    // 连接数据库
    let pool = PgPool::connect(&cfg.database_url)
        .await
        .expect("无法连接数据库");

    // 执行迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("数据库迁移失败");

    tracing::info!("数据库迁移完成");

    let state = Arc::new(AppState { db: pool, config: cfg.clone() });

    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/health", get(|| async { "ok" }))
        // 应用商店 API
        .nest("/store", handlers::store_routes(state.clone()))
        // 中间件
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("SA-Store API 启动 → http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
