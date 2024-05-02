CREATE TABLE IF NOT EXISTS "providers" ( -- 服务商
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "renew_days" INTEGER NOT NULL, -- 续期天数
    "notify_days" INTEGER NOT NULL, -- 通知天数
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "vpss" ( -- VPS
    "id" CHAR(20) PRIMARY KEY,
    "provider_id" CHAR(20) NOT NULL, -- 服务商ID
    "name" VARCHAR(50) NOT NULL, -- 名称
    "expire" TIMESTAMPTZ NOT NULL, -- 过去时间
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "users" ( -- 用户
    "id" CHAR(20) PRIMARY KEY,
    "username" VARCHAR(50) NOT NULL, -- 用户名
    "password" VARCHAR(255) NOT NULL, -- 密码
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(username)
);

CREATE OR REPLACE VIEW "v_vps_proiders" AS
SELECT v.id, v.name, v.provider_id, v.expire,v.dateline,p.name AS provider_name,p.renew_days,p.notify_days
FROM "vpss" AS v
INNER JOIN "providers" AS p
ON v.provider_id = p.id;

INSERT INTO users(id,username,password,dateline) VALUES ('cnv6hv4drfapni9tr4p0', 'axum.rs', '$2b$12$.meDv8XTuGC.HgYLSKohlejbsK80V2MhA/J.m838uIp88ytjd8qtS', '2024-03-23 05:30:36.310579+00');