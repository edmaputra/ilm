CREATE TABLE IF NOT EXISTS project (
    id             VARCHAR(255) PRIMARY KEY,
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    flow_stages_id VARCHAR(255),
    created_at     INTEGER,
    created_by     VARCHAR(255),
    updated_at     INTEGER
);
