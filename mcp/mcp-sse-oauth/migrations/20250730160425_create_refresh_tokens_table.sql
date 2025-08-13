-- Simplified refresh tokens table  
CREATE TABLE refresh_tokens (
    token VARCHAR(128) PRIMARY KEY,
    client_id VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES mcp_clients(client_id) ON DELETE CASCADE
);