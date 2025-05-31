
CREATE TABLE IF NOT EXISTS apps (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    name TEXT NOT NULL,
    repo_url TEXT,
    branch TEXT,
    created_by_id TEXT NOT NULL ,
    app_type TEXT NOT NULL,
    docker_file_path TEXT,
    docker_image TEXT,
    env_vars TEXT,
    build_command TEXT,
    run_command TEXT,
    domain TEXT UNIQUE,
    port INTEGER CHECK (port > 0 AND port < 65536),
    auto_deploy INTEGER NOT NULL DEFAULT 1 CHECK(auto_deploy IN (0,1)),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE SET NULL
);


CREATE INDEX idx_apps_project_id ON apps(project_id);
CREATE INDEX idx_apps_created_by_id ON apps(created_by_id);
CREATE INDEX idx_apps_domain ON apps(domain);


CREATE TRIGGER IF NOT EXISTS update_apps_timestamp 
   AFTER UPDATE ON apps
BEGIN
   UPDATE apps SET updated_at = datetime('now')
   WHERE id = NEW.id;
END;