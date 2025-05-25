
CREATE TABLE IF NOT EXISTS apps (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    branch TEXT NOT NULL,
    created_by_id TEXT NOT NULL,
    app_type TEXT NOT NULL,
    docker_file_path TEXT,
    env_vars TEXT, -- JSON string of environment variables
    build_command TEXT,
    run_command TEXT,
    port INTEGER,
    auto_deploy INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE SET NULL
);