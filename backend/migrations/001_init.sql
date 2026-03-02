-- SA-Store 初始数据库结构
-- 应用注册表 + 版本表 + 下载日志

-- 应用主表
CREATE TABLE IF NOT EXISTS apps (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_key         VARCHAR(50) UNIQUE NOT NULL,
    name            VARCHAR(100) NOT NULL,
    description     TEXT,
    logo_url        VARCHAR(500),
    screenshots     JSONB DEFAULT '[]',
    category        VARCHAR(30) DEFAULT 'tool',
    developer_id    UUID NOT NULL,
    is_official     BOOLEAN DEFAULT false,
    status          VARCHAR(20) DEFAULT 'draft',
    total_downloads BIGINT DEFAULT 0,
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);

-- 版本表
CREATE TABLE IF NOT EXISTS app_versions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id          UUID NOT NULL REFERENCES apps(id) ON DELETE CASCADE,
    version         VARCHAR(20) NOT NULL,
    changelog       TEXT,
    download_url    VARCHAR(500) NOT NULL,
    file_size       BIGINT DEFAULT 0,
    file_hash       VARCHAR(128),
    signature       TEXT,
    platform        VARCHAR(30) DEFAULT 'windows-x86_64',
    status          VARCHAR(20) DEFAULT 'pending',
    review_note     TEXT,
    created_at      TIMESTAMPTZ DEFAULT now(),
    reviewed_at     TIMESTAMPTZ,
    UNIQUE(app_id, version, platform)
);

-- 下载日志
CREATE TABLE IF NOT EXISTS download_logs (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id          UUID NOT NULL REFERENCES apps(id) ON DELETE CASCADE,
    version_id      UUID REFERENCES app_versions(id) ON DELETE SET NULL,
    user_id         UUID,
    ip_address      VARCHAR(45),
    created_at      TIMESTAMPTZ DEFAULT now()
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_apps_status ON apps(status);
CREATE INDEX IF NOT EXISTS idx_apps_category ON apps(category);
CREATE INDEX IF NOT EXISTS idx_app_versions_app_id ON app_versions(app_id);
CREATE INDEX IF NOT EXISTS idx_app_versions_status ON app_versions(status);
CREATE INDEX IF NOT EXISTS idx_download_logs_app_id ON download_logs(app_id);
CREATE INDEX IF NOT EXISTS idx_download_logs_created ON download_logs(created_at);
