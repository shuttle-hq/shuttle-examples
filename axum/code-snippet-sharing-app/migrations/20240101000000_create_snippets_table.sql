CREATE TABLE IF NOT EXISTS snippets (
    id VARCHAR PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    language VARCHAR NOT NULL,
    title VARCHAR,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ,
    view_count INTEGER NOT NULL DEFAULT 0,
    is_public BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX IF NOT EXISTS idx_snippets_created_at ON snippets(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_snippets_language ON snippets(language);
CREATE INDEX IF NOT EXISTS idx_snippets_is_public ON snippets(is_public);
CREATE INDEX IF NOT EXISTS idx_snippets_expires_at ON snippets(expires_at);