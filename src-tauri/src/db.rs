use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use chrono::{DateTime, Utc};

use crate::analysis::RepoAnalysis;
use crate::storage::{ProjectSummary, Task, TaskList};

pub type DbPool = Pool<SqliteConnectionManager>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub path: String,
    pub name: String,
    pub description: Option<String>,
    pub is_git_repo: bool,
    pub is_favorite: bool,
    pub last_analyzed_at: Option<DateTime<Utc>>,
    pub file_count: i64,
    pub total_size_bytes: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: i64,
    pub project_id: i64,
    pub path: String,
    pub relative_path: String,
    pub language: Option<String>,
    pub size_bytes: i64,
    pub lines: Option<i64>,
    pub last_modified: DateTime<Utc>,
    pub content_hash: Option<String>,
    pub analyzed: bool,
}

pub fn init_db_pool(db_path: &Path) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)?;
    
    // Initialize schema
    let conn = pool.get()?;
    init_schema(&conn)?;
    
    Ok(pool)
}

fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA temp_store = MEMORY;
        PRAGMA mmap_size = 30000000000;
        PRAGMA cache_size = -64000;
    ")?;

    // Projects table
    conn.execute("
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            is_git_repo BOOLEAN DEFAULT FALSE,
            is_favorite BOOLEAN DEFAULT FALSE,
            last_analyzed_at TIMESTAMP,
            file_count INTEGER DEFAULT 0,
            total_size_bytes INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    ", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_projects_path ON projects(path)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_projects_favorite ON projects(is_favorite)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_projects_updated ON projects(updated_at DESC)", [])?;

    // Files table
    conn.execute("
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            path TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            language TEXT,
            size_bytes INTEGER,
            lines INTEGER,
            last_modified TIMESTAMP,
            content_hash TEXT,
            analyzed BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id, path)
        )
    ", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_files_project ON files(project_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_files_language ON files(language)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_files_modified ON files(last_modified DESC)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_files_size ON files(size_bytes DESC)", [])?;

    // Analysis cache table
    conn.execute("
        CREATE TABLE IF NOT EXISTS analysis_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            analysis_data BLOB,
            technologies TEXT,
            metrics TEXT,
            cached_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            expires_at TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id)
        )
    ", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_analysis_expires ON analysis_cache(expires_at)", [])?;

    // Tasks table
    conn.execute("
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            project_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            description TEXT,
            priority INTEGER DEFAULT 0,
            completed BOOLEAN DEFAULT FALSE,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            completed_at TIMESTAMP,
            due_date TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )
    ", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_due ON tasks(due_date)", [])?;

    // Summaries table
    conn.execute("
        CREATE TABLE IF NOT EXISTS summaries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            summary_text TEXT NOT NULL,
            key_features TEXT,
            technologies TEXT,
            generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id)
        )
    ", [])?;

    // Git info table
    conn.execute("
        CREATE TABLE IF NOT EXISTS git_info (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            current_branch TEXT,
            commit_count INTEGER,
            remotes TEXT,
            last_commit_date TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id)
        )
    ", [])?;

    // Settings table
    conn.execute("
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    ", [])?;

    Ok(())
}

// Project operations
pub fn upsert_project(
    conn: &Connection,
    path: &str,
    name: &str,
    description: Option<&str>,
    is_git_repo: bool,
) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO projects (path, name, description, is_git_repo, updated_at)
         VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
         ON CONFLICT(path) DO UPDATE SET
            name = excluded.name,
            description = excluded.description,
            is_git_repo = excluded.is_git_repo,
            updated_at = CURRENT_TIMESTAMP",
        params![path, name, description, is_git_repo],
    )?;
    
    Ok(conn.last_insert_rowid())
}

pub fn get_project_by_path(
    conn: &Connection,
    path: &str,
) -> Result<Option<Project>, rusqlite::Error> {
    conn.query_row(
        "SELECT id, path, name, description, is_git_repo, is_favorite, 
                last_analyzed_at, file_count, total_size_bytes, created_at, updated_at
         FROM projects WHERE path = ?1",
        params![path],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                is_git_repo: row.get(4)?,
                is_favorite: row.get(5)?,
                last_analyzed_at: row.get(6)?,
                file_count: row.get(7)?,
                total_size_bytes: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        },
    ).optional()
}

pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, description, is_git_repo, is_favorite, 
                last_analyzed_at, file_count, total_size_bytes, created_at, updated_at
         FROM projects 
         ORDER BY is_favorite DESC, updated_at DESC",
    )?;

    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            path: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            is_git_repo: row.get(4)?,
            is_favorite: row.get(5)?,
            last_analyzed_at: row.get(6)?,
            file_count: row.get(7)?,
            total_size_bytes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

pub fn update_project_file_count(
    conn: &Connection,
    project_id: i64,
    count: i64,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE projects SET file_count = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        params![count, project_id],
    )?;
    Ok(())
}

pub fn toggle_favorite(
    conn: &Connection,
    project_path: &str,
    is_favorite: bool,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE projects SET is_favorite = ?1, updated_at = CURRENT_TIMESTAMP WHERE path = ?2",
        params![is_favorite, project_path],
    )?;
    Ok(())
}

pub fn get_favorites(conn: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT path FROM projects WHERE is_favorite = TRUE")?;
    let paths = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(paths)
}

// Analysis cache operations
pub fn cache_analysis(
    conn: &Connection,
    project_id: i64,
    analysis: &RepoAnalysis,
    ttl_hours: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let analysis_blob = bincode::serialize(analysis)?;
    let technologies = analysis.technologies.join(",");
    let metrics = serde_json::to_string(&analysis.metrics)?;
    
    conn.execute(
        "INSERT OR REPLACE INTO analysis_cache 
         (project_id, analysis_data, technologies, metrics, cached_at, expires_at)
         VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP, datetime('now', '+' || ?5 || ' hours'))",
        params![project_id, analysis_blob, technologies, metrics, ttl_hours],
    )?;
    
    // Update last analyzed timestamp
    conn.execute(
        "UPDATE projects SET last_analyzed_at = CURRENT_TIMESTAMP WHERE id = ?1",
        params![project_id],
    )?;
    
    Ok(())
}

pub fn get_cached_analysis(
    conn: &Connection,
    project_id: i64,
) -> Result<Option<RepoAnalysis>, Box<dyn std::error::Error>> {
    let result: Option<Vec<u8>> = conn.query_row(
        "SELECT analysis_data FROM analysis_cache 
         WHERE project_id = ?1 AND expires_at > CURRENT_TIMESTAMP",
        params![project_id],
        |row| row.get(0),
    ).optional()?;
    
    if let Some(data) = result {
        let analysis: RepoAnalysis = bincode::deserialize(&data)?;
        Ok(Some(analysis))
    } else {
        Ok(None)
    }
}

// File operations
pub fn batch_upsert_files(
    tx: &Transaction,
    project_id: i64,
    files: &[FileMetadata],
) -> Result<(), rusqlite::Error> {
    let mut stmt = tx.prepare(
        "INSERT INTO files (project_id, path, relative_path, language, size_bytes, 
                           lines, last_modified, content_hash, analyzed)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(project_id, path) DO UPDATE SET
            size_bytes = excluded.size_bytes,
            last_modified = excluded.last_modified,
            content_hash = excluded.content_hash,
            analyzed = excluded.analyzed"
    )?;
    
    for file in files {
        stmt.execute(params![
            project_id,
            file.path,
            file.relative_path,
            file.language,
            file.size_bytes,
            file.lines,
            file.last_modified,
            file.content_hash,
            file.analyzed
        ])?;
    }
    
    // Update project statistics
    tx.execute(
        "UPDATE projects SET 
            file_count = (SELECT COUNT(*) FROM files WHERE project_id = ?1),
            total_size_bytes = (SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE project_id = ?1),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = ?1",
        params![project_id],
    )?;
    
    Ok(())
}

// Task operations
pub fn save_task_list(
    conn: &Connection,
    project_id: i64,
    tasks: &[Task],
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = conn.unchecked_transaction()?;
    
    // Clear existing tasks for this project
    tx.execute("DELETE FROM tasks WHERE project_id = ?1", params![project_id])?;
    
    // Insert new tasks
    let mut stmt = tx.prepare(
        "INSERT INTO tasks (id, project_id, text, completed, created_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )?;
    
    for task in tasks {
        let created_at = DateTime::parse_from_rfc3339(&task.created_at)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
        let completed_at = task.completed_at.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        stmt.execute(params![
            task.id,
            project_id,
            task.text,
            task.completed,
            created_at,
            completed_at
        ])?;
    }
    
    // Ensure statement is dropped before committing the transaction
    drop(stmt);
    tx.commit()?;
    Ok(())
}

pub fn load_task_list(
    conn: &Connection,
    project_id: i64,
    project_path: &str,
) -> Result<Option<TaskList>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, text, completed, created_at, completed_at 
         FROM tasks WHERE project_id = ?1 ORDER BY created_at DESC"
    )?;
    
    let tasks: Vec<Task> = stmt.query_map(params![project_id], |row| {
        let created_at: DateTime<Utc> = row.get(3)?;
        let completed_at: Option<DateTime<Utc>> = row.get(4)?;
        
        Ok(Task {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get(2)?,
            created_at: created_at.to_rfc3339(),
            completed_at: completed_at.map(|dt| dt.to_rfc3339()),
        })
    })?.collect::<Result<Vec<_>, _>>()?;
    
    if tasks.is_empty() {
        Ok(None)
    } else {
        Ok(Some(TaskList {
            project_path: project_path.to_string(),
            tasks,
            updated_at: Utc::now().to_rfc3339(),
        }))
    }
}

// Summary operations
pub fn save_summary(
    conn: &Connection,
    project_id: i64,
    summary: &ProjectSummary,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_features = serde_json::to_string(&summary.key_features)?;
    let technologies = serde_json::to_string(&summary.technologies)?;
    
    conn.execute(
        "INSERT OR REPLACE INTO summaries 
         (project_id, summary_text, key_features, technologies, generated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            project_id,
            summary.summary,
            key_features,
            technologies,
            summary.generated_at
        ],
    )?;
    
    Ok(())
}

pub fn load_summary(
    conn: &Connection,
    project_id: i64,
    project_path: &str,
) -> Result<Option<ProjectSummary>, Box<dyn std::error::Error>> {
    let result = conn.query_row(
        "SELECT summary_text, key_features, technologies, generated_at 
         FROM summaries WHERE project_id = ?1",
        params![project_id],
        |row| {
            let key_features_str: String = row.get(1)?;
            let technologies_str: String = row.get(2)?;
            
            Ok((
                row.get::<_, String>(0)?,
                key_features_str,
                technologies_str,
                row.get::<_, String>(3)?,
            ))
        },
    ).optional()?;
    
    if let Some((summary, kf_str, tech_str, generated_at)) = result {
        let key_features: Vec<String> = serde_json::from_str(&kf_str)?;
        let technologies: Vec<String> = serde_json::from_str(&tech_str)?;
        
        Ok(Some(ProjectSummary {
            project_path: project_path.to_string(),
            summary,
            generated_at,
            technologies,
            key_features,
        }))
    } else {
        Ok(None)
    }
}

// Settings operations
pub fn save_setting(
    conn: &Connection,
    key: &str,
    value: &str,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) 
         VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![key, value],
    )?;
    Ok(())
}

pub fn load_setting(
    conn: &Connection,
    key: &str,
) -> Result<Option<String>, rusqlite::Error> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    ).optional()
}

// Utility functions
pub fn clear_expired_cache(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM analysis_cache WHERE expires_at < CURRENT_TIMESTAMP", [])
}

pub fn get_project_statistics(conn: &Connection) -> Result<(i64, i64, i64), rusqlite::Error> {
    conn.query_row(
        "SELECT 
            COUNT(*) as project_count,
            COALESCE(SUM(file_count), 0) as total_files,
            COALESCE(SUM(total_size_bytes), 0) as total_size
         FROM projects",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
}
