-- Simplified authorization codes table
CREATE TABLE authorization_codes (
    code VARCHAR(128) PRIMARY KEY,
    client_id VARCHAR(255) NOT NULL,
    redirect_uri TEXT NOT NULL,
    code_challenge VARCHAR(128) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES mcp_clients(client_id) ON DELETE CASCADE
);