-- SA-Store 开发者认证体系
-- 符合国内应用商店监管要求

-- 开发者档案表（关联 sa-user 用户）
CREATE TABLE IF NOT EXISTS developers (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL UNIQUE,       -- 关联 sa-user 用户ID
    type                VARCHAR(20) NOT NULL,       -- 'individual' 个人 | 'enterprise' 企业
    status              VARCHAR(20) DEFAULT 'pending', -- pending/verified/rejected/suspended
    
    -- ── 通用信息 ──
    contact_name        VARCHAR(50) NOT NULL,       -- 联系人姓名
    contact_phone       VARCHAR(20) NOT NULL,       -- 联系电话
    contact_email       VARCHAR(100) NOT NULL,      -- 联系邮箱
    
    -- ── 个人开发者 ──
    id_card_name        VARCHAR(50),                -- 身份证姓名
    id_card_number      VARCHAR(18),                -- 身份证号码
    id_card_front_url   VARCHAR(500),               -- 身份证正面照
    id_card_back_url    VARCHAR(500),               -- 身份证背面照
    
    -- ── 企业开发者 ──
    company_name        VARCHAR(200),               -- 企业全称
    credit_code         VARCHAR(18),                -- 统一社会信用代码
    business_license_url VARCHAR(500),              -- 营业执照扫描件
    legal_person_name   VARCHAR(50),                -- 法定代表人姓名
    legal_person_id     VARCHAR(18),                -- 法人身份证号
    legal_person_id_front_url VARCHAR(500),          -- 法人身份证正面照
    legal_person_id_back_url  VARCHAR(500),          -- 法人身份证背面照
    authorization_url   VARCHAR(500),               -- 授权委托书（如操作人非法人）
    
    -- ── 附加资质 ──
    icp_license         VARCHAR(50),                -- ICP备案号
    app_category_license_url VARCHAR(500),           -- 行业资质证明（如金融、医疗等特殊行业）
    
    -- ── 审核信息 ──
    reject_reason       TEXT,                       -- 驳回原因
    verified_at         TIMESTAMPTZ,                -- 认证通过时间
    reviewer_id         UUID,                       -- 审核人
    
    created_at          TIMESTAMPTZ DEFAULT now(),
    updated_at          TIMESTAMPTZ DEFAULT now()
);

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

-- 审核日志表（留痕）
CREATE TABLE IF NOT EXISTS review_logs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    target_type         VARCHAR(20) NOT NULL,       -- 'developer' | 'app' | 'version'
    target_id           UUID NOT NULL,              -- 对应记录ID
    action              VARCHAR(20) NOT NULL,       -- 'approve' | 'reject' | 'suspend'
    reason              TEXT,
    reviewer_id         UUID NOT NULL,              -- 审核人 (sa-user)
    created_at          TIMESTAMPTZ DEFAULT now()
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_developers_user_id ON developers(user_id);
CREATE INDEX IF NOT EXISTS idx_developers_status ON developers(status);
CREATE INDEX IF NOT EXISTS idx_developers_type ON developers(type);
CREATE INDEX IF NOT EXISTS idx_app_compliance_app_id ON app_compliance(app_id);
CREATE INDEX IF NOT EXISTS idx_review_logs_target ON review_logs(target_type, target_id);

-- 修改 apps 表，关联开发者
ALTER TABLE apps ADD COLUMN IF NOT EXISTS developer_verified BOOLEAN DEFAULT false;
