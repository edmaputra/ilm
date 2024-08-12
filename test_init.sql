CREATE USER project_user WITH PASSWORD 'project_pwd';
CREATE USER task_user WITH PASSWORD 'task_pwd';

GRANT ALL PRIVILEGES ON DATABASE ilm TO project_user;
GRANT ALL PRIVILEGES ON DATABASE ilm TO task_user;

CREATE TABLE IF NOT EXISTS project (
    id             VARCHAR(255) PRIMARY KEY,
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    flow_stages_id VARCHAR(255),
    created_at     INTEGER,
    created_by     VARCHAR(255),
    updated_at     INTEGER
);

INSERT INTO project (id, name, description, flow_stages_id, created_at, created_by, updated_at)
VALUES
    ('1', 'Project 1', 'Description 1', 'flow_stages_1', 1631232000, 'User 1', 1631232000),
    ('2', 'Project 2', 'Description 2', 'flow_stages_2', 1631232000, 'User 2', 1631232000),
    ('3', 'Project 3', 'Description 3', 'flow_stages_3', 1631232000, 'User 3', 1631232000);

