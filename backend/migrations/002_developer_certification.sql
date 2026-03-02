-- SA-Store 应用合规信息（开发者认证已迁移到 sa-user）
-- 此表仅保留商店特有的应用合规审核数据

-- 应用合规信息表
CREATE TABLE IF NOT EXISTS app_compliance (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id              UUID NOT NULL REFERENCES apps(id) ON DELETE CASCADE,

    -- ── 隐私与安全 ──
    privacy_policy_url  VARCHAR(500),               -- 隐私政策链接
    user_agreement_url  VARCHAR(500),               -- 用户协议链接

    -- ── 权限说明 ──
    permissions_desc    TEXT,                       -- 权限使用说明
    data_collection     JSONB DEFAULT '[]',         -- 收集的用户数据类型列表

    -- ── 内容分级 ──
    age_rating          VARCHAR(10) DEFAULT '0+',   -- 年龄分级: 0+ 4+ 9+ 12+ 17+
    has_ads             BOOLEAN DEFAULT false,      -- 是否含广告
    has_iap             BOOLEAN DEFAULT false,      -- 是否含内购

    -- ── 审核记录 ──
    review_status       VARCHAR(20) DEFAULT 'pending',
    review_note         TEXT,
    reviewed_at         TIMESTAMPTZ,
    reviewer_id         UUID,

    created_at          TIMESTAMPTZ DEFAULT now(),
    updated_at          TIMESTAMPTZ DEFAULT now(),

    UNIQUE(app_id)
);

-- 应用审核日志（商店维度）
CREATE TABLE IF NOT EXISTS app_review_logs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    target_type         VARCHAR(20) NOT NULL,       -- 'app' | 'version'
    target_id           UUID NOT NULL,
    action              VARCHAR(20) NOT NULL,       -- 'approve' | 'reject' | 'suspend' | 'takedown'
    reason              TEXT,
    reviewer_id         UUID NOT NULL,
    created_at          TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_app_compliance_app_id ON app_compliance(app_id);
CREATE INDEX IF NOT EXISTS idx_app_review_logs_target ON app_review_logs(target_type, target_id);

-- 修改 apps 表，关联 sa-user 的认证状态
ALTER TABLE apps ADD COLUMN IF NOT EXISTS developer_verified BOOLEAN DEFAULT false;
