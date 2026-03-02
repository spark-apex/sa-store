/// 配置管理 — 从环境变量读取
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://sa_store:sa_store_dev@localhost:5437/sa_store".into()),
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3011),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-jwt-secret".into()),
        }
    }
}
