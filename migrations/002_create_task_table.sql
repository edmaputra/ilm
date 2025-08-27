-- Create task status enum
CREATE TYPE task_status AS ENUM ('todo', 'in_progress', 'done', 'cancelled');

-- Create task priority enum  
CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high', 'critical');

-- Create task table
CREATE TABLE IF NOT EXISTS task (
    id             UUID PRIMARY KEY,
    project_id     UUID NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    title          VARCHAR(255) NOT NULL,
    description    TEXT,
    status         task_status NOT NULL DEFAULT 'todo',
    priority       task_priority NOT NULL DEFAULT 'medium',
    assigned_to    VARCHAR(255),
    due_date       TIMESTAMPTZ,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by     VARCHAR(255) NOT NULL,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_task_project_id ON task(project_id);
CREATE INDEX idx_task_status ON task(status);
CREATE INDEX idx_task_assigned_to ON task(assigned_to);
CREATE INDEX idx_task_due_date ON task(due_date);
