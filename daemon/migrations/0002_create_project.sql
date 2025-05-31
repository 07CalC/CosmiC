CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    tags TEXT,
    last_deployment_at TEXT DEFAULT NULL,
    owner_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(owner_id, name)
);

CREATE INDEX idx_projects_owner_id ON projects(owner_id);
CREATE INDEX idx_projects_name ON projects(name);
CREATE INDEX idx_projects_last_deployment ON projects(last_deployment_at);

CREATE TRIGGER IF NOT EXISTS update_projects_timestamp 
   AFTER UPDATE ON projects
BEGIN
   UPDATE projects SET updated_at = datetime('now')
   WHERE id = NEW.id;
END;