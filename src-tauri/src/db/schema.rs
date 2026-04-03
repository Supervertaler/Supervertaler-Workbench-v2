/// SQL schema for translation memory and termbase databases.
/// Uses SQLite with FTS5 for fast concordance search.

pub const TM_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS tm_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_text TEXT NOT NULL,
    target_text TEXT NOT NULL,
    source_language TEXT NOT NULL,
    target_language TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    modified_at TEXT DEFAULT (datetime('now')),
    context TEXT,
    origin TEXT
);

CREATE VIRTUAL TABLE IF NOT EXISTS tm_fts USING fts5(
    source_text,
    target_text,
    content='tm_entries',
    content_rowid='id'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS tm_ai AFTER INSERT ON tm_entries BEGIN
    INSERT INTO tm_fts(rowid, source_text, target_text)
    VALUES (new.id, new.source_text, new.target_text);
END;

CREATE TRIGGER IF NOT EXISTS tm_ad AFTER DELETE ON tm_entries BEGIN
    INSERT INTO tm_fts(tm_fts, rowid, source_text, target_text)
    VALUES ('delete', old.id, old.source_text, old.target_text);
END;

CREATE TRIGGER IF NOT EXISTS tm_au AFTER UPDATE ON tm_entries BEGIN
    INSERT INTO tm_fts(tm_fts, rowid, source_text, target_text)
    VALUES ('delete', old.id, old.source_text, old.target_text);
    INSERT INTO tm_fts(rowid, source_text, target_text)
    VALUES (new.id, new.source_text, new.target_text);
END;
"#;

pub const TERMBASE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS terms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_term TEXT NOT NULL,
    target_term TEXT NOT NULL,
    priority INTEGER DEFAULT 50,
    forbidden INTEGER DEFAULT 0,
    notes TEXT,
    domain TEXT,
    termbase_id INTEGER,
    created_at TEXT DEFAULT (datetime('now')),
    modified_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_terms_source ON terms(source_term);
"#;
