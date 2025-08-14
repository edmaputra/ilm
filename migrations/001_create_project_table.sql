-- Create project table
CREATE TABLE IF NOT EXISTS project (
    id             UUID PRIMARY KEY,
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    flow_stages_id VARCHAR(255),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by     VARCHAR(255) NOT NULL,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
