# S3 Upload Manager API

A Rust-based API for managing S3 upload metadata with AWS RDS PostgreSQL database storage. This API stores details about files uploaded to S3 (assuming S3 upload is handled separately).

## Features

- **Create Upload Records**: Store metadata about S3 uploads in PostgreSQL
- **Retrieve Upload Records**: Get all uploads or specific upload by ID
- **AWS RDS Database**: Uses dedicated AWS RDS PostgreSQL database via Shuttle
- **UUID-based**: Each upload record gets a unique UUID identifier
- **Metadata Tracking**: Track filename, S3 key, bucket, URL, file size, content type, and timestamp

## Prerequisites

- Rust (latest stable version)
- Shuttle account (for deployment)
- S3 upload mechanism (handled separately)

## AWS RDS Configuration

This application uses **AWS RDS PostgreSQL** as its database backend. When you deploy with Shuttle, it will automatically provision a dedicated AWS RDS PostgreSQL instance for your application.

### How It Works

- Shuttle's `shuttle-aws-rds` resource automatically provisions an AWS RDS PostgreSQL database
- The database connection is injected into your application at runtime
- Database credentials and connection details are managed securely by Shuttle
- The database is dedicated to your application (not shared with other projects)

### Database Provisioning

On first deployment, Shuttle will:
1. Create a new AWS RDS PostgreSQL instance
2. Configure security groups and networking
3. Provide the connection pool to your application
4. Run the table creation migration automatically

**Note**: AWS RDS provisioning may take a few minutes on first deployment.

## Database Schema

The application automatically creates the following table on startup:

```sql
CREATE TABLE s3_uploads (
    id UUID PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    s3_key VARCHAR(512) NOT NULL,
    s3_bucket VARCHAR(255) NOT NULL,
    s3_url VARCHAR(512) NOT NULL,
    file_size BIGINT NOT NULL,
    content_type VARCHAR(100),
    uploaded_at TIMESTAMPTZ NOT NULL
);
```

## API Endpoints

### 1. Health Check
```
GET /
```
Returns a simple health check message.

**Response:**
```
S3 Upload Manager API is running!
```

### 2. Create Upload Record
```
POST /uploads
```
Store metadata about an S3 upload in the database.

**Request Body:**
```json
{
  "filename": "document.pdf",
  "s3_key": "uploads/2024/document.pdf",
  "s3_bucket": "my-bucket",
  "s3_url": "https://my-bucket.s3.amazonaws.com/uploads/2024/document.pdf",
  "file_size": 1048576,
  "content_type": "application/pdf"
}
```

**Example using curl:**
```bash
curl -X POST http://localhost:8000/uploads \
  -H "Content-Type: application/json" \
  -d '{
    "filename": "document.pdf",
    "s3_key": "uploads/2024/document.pdf",
    "s3_bucket": "my-bucket",
    "s3_url": "https://my-bucket.s3.amazonaws.com/uploads/2024/document.pdf",
    "file_size": 1048576,
    "content_type": "application/pdf"
  }'
```

**Response (201 Created):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "document.pdf",
  "s3_url": "https://my-bucket.s3.amazonaws.com/uploads/2024/document.pdf",
  "uploaded_at": "2024-01-20T10:30:00Z"
}
```

### 3. Get All Upload Records
```
GET /uploads
```
Retrieve a list of all upload records, ordered by most recent first.

**Response:**
```json
{
  "uploads": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "filename": "document.pdf",
      "s3_key": "uploads/2024/document.pdf",
      "s3_bucket": "my-bucket",
      "s3_url": "https://my-bucket.s3.amazonaws.com/uploads/2024/document.pdf",
      "file_size": 1048576,
      "content_type": "application/pdf",
      "uploaded_at": "2024-01-20T10:30:00Z"
    }
  ],
  "count": 1
}
```

### 4. Get Upload Record by ID
```
GET /uploads/:id
```
Retrieve metadata for a specific upload by its UUID.

**Example:**
```bash
curl http://localhost:8000/uploads/550e8400-e29b-41d4-a716-446655440000
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "document.pdf",
  "s3_key": "uploads/2024/document.pdf",
  "s3_bucket": "my-bucket",
  "s3_url": "https://my-bucket.s3.amazonaws.com/uploads/2024/document.pdf",
  "file_size": 1048576,
  "content_type": "application/pdf",
  "uploaded_at": "2024-01-20T10:30:00Z"
}
```

## Request Validation

The create endpoint validates:
- **filename**: Cannot be empty
- **s3_key**: Cannot be empty
- **file_size**: Must be greater than 0

## Error Responses

The API returns appropriate HTTP status codes:

- `200 OK`: Successful GET request
- `201 Created`: Upload record successfully created
- `400 Bad Request`: Invalid request (validation errors)
- `404 Not Found`: Upload record not found
- `500 Internal Server Error`: Server-side errors

Error responses follow this format:
```json
{
  "error": "Error message description"
}
```

## Local Development

1. Install dependencies:
```bash
cargo build
```

2. Run locally with Shuttle:
```bash
cargo shuttle run
```

The API will be available at `http://localhost:8000`

## Deployment with Shuttle

### Using Shuttle CLI

1. Login to Shuttle:
```bash
cargo shuttle login
```

2. Deploy the application:
```bash
cargo shuttle deploy
```

### Using Shuttle MCP Server

You can also use the Shuttle MCP server tools to deploy:

1. List your projects:
```bash
# Use mcp0_project_list tool
```

2. Create a new project (if needed):
```bash
# Use mcp0_project_create tool with project name
```

3. Deploy:
```bash
# Use mcp0_deploy tool with project_id and working directory
```

4. Check deployment status:
```bash
# Use mcp0_deployment_status tool
```

## Integration Example

This API is designed to work with a separate S3 upload mechanism. Here's a typical workflow:

1. **Client uploads file to S3** (using pre-signed URLs, direct upload, etc.)
2. **After successful S3 upload**, client calls `POST /uploads` with the file metadata
3. **API stores the metadata** in PostgreSQL and returns the record ID
4. **Client can retrieve upload records** using `GET /uploads` or `GET /uploads/:id`

## Technology Stack

- **Framework**: Axum (Rust web framework)
- **Database**: PostgreSQL (via AWS RDS)
- **ORM**: SQLx
- **Deployment**: Shuttle
