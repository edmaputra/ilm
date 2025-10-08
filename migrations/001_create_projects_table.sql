-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    owner_id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL
);

-- Create index on owner_id for faster queries
CREATE INDEX IF NOT EXISTS idx_projects_owner_id ON projects(owner_id);

-- Create index on status for filtering
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);

-- Create index on name for search
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);