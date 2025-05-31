CREATE TABLE IF NOT EXISTS deployment_jobs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    app_id TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    branch TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('queued', 'running', 'completed', 'failed', 'cancelled')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    started_at TEXT DEFAULT NULL,
    finished_at TEXT DEFAULT NULL,
    logs_url TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
);

CREATE INDEX idx_deployment_jobs_project ON deployment_jobs(project_id);
CREATE INDEX idx_deployment_jobs_app ON deployment_jobs(app_id);
CREATE INDEX idx_deployment_jobs_status ON deployment_jobs(status);
CREATE INDEX idx_deployment_jobs_created_at ON deployment_jobs(created_at DESC);

CREATE INDEX idx_deployment_jobs_app_status ON deployment_jobs(app_id, status);

CREATE TRIGGER IF NOT EXISTS update_deployment_jobs_started 
   AFTER UPDATE OF status ON deployment_jobs
   WHEN NEW.status = 'running' AND OLD.status = 'queued'
BEGIN
   UPDATE deployment_jobs 
   SET started_at = datetime('now')
   WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_deployment_jobs_finished 
   AFTER UPDATE OF status ON deployment_jobs
   WHEN NEW.status IN ('completed', 'failed', 'cancelled')
BEGIN
   UPDATE deployment_jobs 
   SET finished_at = datetime('now')
   WHERE id = NEW.id;
END;