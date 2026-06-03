CREATE TABLE IF NOT EXISTS workspaces (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL UNIQUE,
    root_path   TEXT    NOT NULL,
    is_active   INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS source_folders (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    path          TEXT    NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE (path)
);

CREATE TABLE IF NOT EXISTS workspace_source_folders (
    workspace_id        INTEGER NOT NULL REFERENCES workspaces(id)     ON DELETE CASCADE,
    source_folder_id    INTEGER NOT NULL REFERENCES source_folders(id) ON DELETE CASCADE,
    enabled             INTEGER NOT NULL DEFAULT 1,
    created_at          TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (workspace_id, source_folder_id)
);

CREATE INDEX IF NOT EXISTS idx_workspace_source_folders_source_folder_id ON workspace_source_folders(source_folder_id);

CREATE TABLE IF NOT EXISTS domains (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id  INTEGER NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name          TEXT    NOT NULL,
    folder_path   TEXT    NOT NULL,
    total_tokens  INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    deleted_at    TEXT,
    UNIQUE (workspace_id, name)
);

CREATE TABLE IF NOT EXISTS vocabulary (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    term                TEXT    NOT NULL UNIQUE,
    document_frequency  INTEGER NOT NULL DEFAULT 0,
    total_count         INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS keywords (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_id   INTEGER NOT NULL REFERENCES domains(id)    ON DELETE CASCADE,
    term_id     INTEGER NOT NULL REFERENCES vocabulary(id) ON DELETE RESTRICT,
    weight      REAL    NOT NULL DEFAULT 1.0,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE (domain_id, term_id)
);

CREATE TABLE IF NOT EXISTS files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id  INTEGER REFERENCES workspaces(id) ON DELETE CASCADE,
    domain_id     INTEGER REFERENCES domains(id)    ON DELETE CASCADE,
    source_folder_id  INTEGER REFERENCES source_folders(id) ON DELETE SET NULL,
    current_path  TEXT    NOT NULL UNIQUE,
    filename      TEXT    NOT NULL,
    extension     TEXT,
    size_bytes    INTEGER NOT NULL,
    total_tokens  INTEGER NOT NULL,
    mime_type     TEXT,
    content_hash  TEXT    NOT NULL,
    modified_at   TEXT    NOT NULL,
    indexed_at        TEXT    NOT NULL DEFAULT (datetime('now')),
    status            TEXT    NOT NULL DEFAULT 'pending',
    CHECK(status IN ('pending', 'classified', 'inbox', 'unclassifiable', 'error'))
);

CREATE INDEX IF NOT EXISTS idx_files_workspace        ON files(workspace_id);
CREATE INDEX IF NOT EXISTS idx_files_domain           ON files(domain_id);
CREATE INDEX IF NOT EXISTS idx_files_hash             ON files(content_hash);
CREATE INDEX IF NOT EXISTS idx_files_source_folder    ON files(source_folder_id);

CREATE TABLE IF NOT EXISTS file_tokens (
    file_id  INTEGER NOT NULL REFERENCES files(id)      ON DELETE CASCADE,
    term_id  INTEGER NOT NULL REFERENCES vocabulary(id) ON DELETE RESTRICT,
    count    INTEGER NOT NULL,
    PRIMARY KEY (file_id, term_id)
);

CREATE INDEX IF NOT EXISTS idx_file_tokens_term_id ON file_tokens(term_id);

CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
    filename,
    content,
    tokenize='unicode61 remove_diacritics 2'
);

CREATE TABLE IF NOT EXISTS class_term_counts (
    domain_id  INTEGER NOT NULL REFERENCES domains(id)    ON DELETE CASCADE,
    term_id    INTEGER NOT NULL REFERENCES vocabulary(id) ON DELETE RESTRICT,
    count      INTEGER NOT NULL,
    PRIMARY KEY (domain_id, term_id)
);

CREATE INDEX IF NOT EXISTS idx_class_term_counts_term_id ON class_term_counts(term_id);

CREATE TABLE IF NOT EXISTS classifications (
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id                INTEGER NOT NULL REFERENCES files(id)   ON DELETE CASCADE,
    source_path            TEXT    NOT NULL,
    destination_domain_id  INTEGER NOT NULL REFERENCES domains(id) ON DELETE CASCADE,
    confidence             REAL    NOT NULL,
    model_version          TEXT    NOT NULL,
    classified_at          TEXT    NOT NULL DEFAULT (datetime('now')),
    CHECK(confidence BETWEEN 0.0 AND 1.0)
);

CREATE INDEX IF NOT EXISTS idx_classifications_file ON classifications(file_id);
CREATE INDEX IF NOT EXISTS idx_classifications_time ON classifications(classified_at);
