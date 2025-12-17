# Actix-web REST API with MySQL

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-ready REST API template built with Actix-web, MySQL, and OpenAPI documentation. This project demonstrates best practices for building scalable, maintainable web services in Rust.

## Table of Contents

- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
  - [Option 1: Docker Compose (Recommended)](#option-1-docker-compose-recommended)
  - [Option 2: Local MySQL](#option-2-local-mysql)
- [Docker Deployment](#docker-deployment)
- [API Documentation](#api-documentation)
- [API Endpoints](#api-endpoints)
- [Error Handling](#error-handling)
- [Architecture](#architecture)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Features

- ✨ RESTful API with full CRUD operations
- 🗄️ MySQL database integration with connection pooling
- 📚 OpenAPI 3.0 specification with interactive Swagger UI
- 🛡️ Comprehensive error handling with custom error types
- ✅ Advanced request validation with detailed field-level error messages
  - Email format validation
  - Name length constraints (1-100 characters)
  - Email length constraints (max 255 characters)
- 🔀 Smart trailing slash handling for API routes (follows REST best practices)
- 🌐 CORS support for cross-origin requests
- 📝 Structured logging middleware
- 🏗️ Clean architecture with clear separation of concerns
- ⚡ Async/await throughout for optimal performance
- 🔧 Environment-based configuration
- 🐳 Docker support with optimized multi-stage builds
- 🛠️ Makefile commands for streamlined development workflow

## Technology Stack

- **[Actix-web](https://actix.rs/)** - Fast, pragmatic web framework for Rust
- **[SQLx](https://github.com/launchbadge/sqlx)** - Async, pure Rust SQL toolkit with compile-time checked queries
- **[utoipa](https://github.com/juhaku/utoipa)** - Auto-generated OpenAPI documentation from code
- **[validator](https://github.com/Keats/validator)** - Struct validation with custom error messages
- **[Serde](https://serde.rs/)** - Serialization/deserialization framework
- **[Tokio](https://tokio.rs/)** - Async runtime for Rust
- **MySQL 8.0+** - Relational database
- **Docker & Docker Compose** - Containerization and orchestration

## Project Structure

```
hello-actix/
├── src/
│   ├── config/          # Configuration (database connection)
│   ├── models/          # Database entities
│   ├── dto/             # Data Transfer Objects (API request/response)
│   ├── db/              # Database queries
│   ├── services/        # Business logic
│   ├── handlers/        # HTTP handlers (controllers)
│   ├── routes/          # Route configuration
│   ├── errors/          # Error handling
│   └── main.rs          # Application entry point
├── migrations/          # SQL migration files
└── Cargo.toml
```

## Prerequisites

### For Docker (Recommended)
- **Docker** 20.10 or higher
- **Docker Compose** 2.0 or higher

### For Local Development
- **Rust** 1.80 or higher ([install here](https://www.rust-lang.org/tools/install))
- **MySQL** 8.0 or higher
- **cargo** (comes with Rust)

## Quick Start

### Option 1: Docker Compose (Recommended)

This is the easiest way to get started. Everything runs in containers with no local MySQL installation required.

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/hello-actix.git
   cd hello-actix
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```

   The defaults in `.env.example` work out of the box for Docker Compose!

3. **Start the application**
   ```bash
   docker-compose up -d
   ```

   This will:
   - Pull the MySQL 8.0 image
   - Build your Rust application
   - Run database migrations automatically
   - Start both services

4. **View logs**
   ```bash
   docker-compose logs -f app
   ```

5. **Access the application**
   - API: http://localhost:8080
   - Swagger UI: http://localhost:8080/swagger-ui/
   - Health check: http://localhost:8080/health

6. **Stop the application**
   ```bash
   docker-compose down
   ```

   To also remove volumes (database data):
   ```bash
   docker-compose down -v
   ```

### Option 2: Local MySQL

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/hello-actix.git
   cd hello-actix
   ```

2. **Create a MySQL database**
   ```bash
   mysql -u root -p
   ```
   ```sql
   CREATE DATABASE rust;
   exit;
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your database credentials:
   ```env
   DATABASE_URL=mysql://root:password@127.0.0.1:3306/rust
   HOST=127.0.0.1
   PORT=8080
   RUST_LOG=info
   ```

   > **Note**: For MySQL with no password, use `mysql://root:@127.0.0.1:3306/rust`

4. **Run database migrations**
   ```bash
   mysql -u root -p rust < migrations/01_create_users_table.sql
   ```

5. **Build and run**
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8080`

## Docker Deployment

### Development Environment

Use the standard `docker-compose.yml` for development:

```bash
# Start services
docker-compose up -d

# Rebuild after code changes
docker-compose up -d --build

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Production Environment

For production deployments, use `docker-compose.prod.yml`:

1. **Create production environment file**
   ```bash
   cp .env.example .env.prod
   ```

   Update `.env.prod` with secure credentials:
   ```env
   DB_USER=produser
   DB_PASSWORD=<strong-password>
   DB_NAME=rust_prod
   DB_ROOT_PASSWORD=<strong-root-password>
   RUST_LOG=warn
   APP_PORT=8080
   ```

2. **Deploy with production configuration**
   ```bash
   docker-compose -f docker-compose.prod.yml --env-file .env.prod up -d
   ```

3. **View production logs**
   ```bash
   docker-compose -f docker-compose.prod.yml logs -f
   ```

### Docker Commands Reference

```bash
# Build only
docker-compose build

# Run migrations manually
docker-compose exec db mysql -uroot -p${DB_ROOT_PASSWORD} ${DB_NAME} < migrations/01_create_users_table.sql

# Access MySQL shell
docker-compose exec db mysql -u${DB_USER} -p${DB_PASSWORD} ${DB_NAME}

# Access application container
docker-compose exec app sh

# Check service status
docker-compose ps

# Remove everything (including volumes)
docker-compose down -v
```

### Multi-Stage Build Benefits

The Dockerfile uses a multi-stage build which:
- **Reduces image size**: Final image is ~100MB vs ~2GB
- **Improves security**: Only runtime dependencies included
- **Faster deployments**: Smaller images transfer faster
- **Layer caching**: Dependency layer cached separately from code

## API Documentation

Once the server is running, you can explore the API using:

- **Swagger UI**: [http://127.0.0.1:8080/swagger-ui/](http://127.0.0.1:8080/swagger-ui/)
- **OpenAPI JSON**: [http://127.0.0.1:8080/api-docs/openapi.json](http://127.0.0.1:8080/api-docs/openapi.json)

The Swagger UI provides an interactive interface to test all endpoints directly from your browser.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check endpoint |
| `POST` | `/api/users` | Create a new user |
| `GET` | `/api/users` | Get all users |
| `GET` | `/api/users/{id}` | Get a specific user by ID |
| `PUT` | `/api/users/{id}` | Update an existing user |
| `DELETE` | `/api/users/{id}` | Delete a user |

### Trailing Slash Handling

All `/api/*` routes support trailing slashes for convenience:
- ✅ `/api/users` - Canonical URL (recommended)
- ✅ `/api/users/` - Also works (normalized to `/api/users`)
- ✅ `/api/users/1` - Get user by ID
- ✅ `/api/users/1/` - Also works (normalized to `/api/users/1`)

This follows REST API best practices where the canonical URL has no trailing slash, but both variants are accepted for better client compatibility.

## Example Requests

### Create a User
```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com"
  }'
```

**Success Response (201 Created):**
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Validation Error Response (400 Bad Request):**
```json
{
  "error": "VALIDATION_ERROR",
  "errors": [
    {
      "field": "email",
      "message": "Invalid email format"
    },
    {
      "field": "name",
      "message": "Name must be between 1 and 100 characters"
    }
  ]
}
```

### Get All Users
```bash
curl http://127.0.0.1:8080/api/users
```

**Response:**
```json
[
  {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z"
  }
]
```

### Get a User by ID
```bash
curl http://127.0.0.1:8080/api/users/1
```

**Success Response:**
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Response (404 Not Found):**
```json
{
  "error": "NOT_FOUND",
  "message": "User not found"
}
```

### Update a User
```bash
curl -X PUT http://127.0.0.1:8080/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Doe",
    "email": "jane@example.com"
  }'
```

### Delete a User
```bash
curl -X DELETE http://127.0.0.1:8080/api/users/1
```

## Error Handling

The API provides structured error responses for different error scenarios:

### Error Response Types

#### Standard Error Response
For most errors (database, not found, bad request, internal server):
```json
{
  "error": "ERROR_TYPE",
  "message": "Descriptive error message"
}
```

#### Validation Error Response
For request validation failures with detailed field-level errors:
```json
{
  "error": "VALIDATION_ERROR",
  "errors": [
    {
      "field": "field_name",
      "message": "Specific validation message"
    }
  ]
}
```

### Error Types

| Status Code | Error Type | Description |
|-------------|------------|-------------|
| `400` | `BAD_REQUEST` | Invalid request format or parameters |
| `400` | `VALIDATION_ERROR` | Request validation failed (with field details) |
| `404` | `NOT_FOUND` | Resource not found |
| `500` | `DATABASE_ERROR` | Database operation failed |
| `500` | `INTERNAL_SERVER_ERROR` | Unexpected server error |

### Validation Rules

- **Email**: Must be a valid email format (max 255 characters)
- **Name**: Required, 1-100 characters

Example validation error for invalid input:
```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "", "email": "invalid-email"}'
```

Response:
```json
{
  "error": "VALIDATION_ERROR",
  "errors": [
    {
      "field": "name",
      "message": "Name must be between 1 and 100 characters"
    },
    {
      "field": "email",
      "message": "Invalid email format"
    }
  ]
}
```

## Architecture

This project follows a **clean architecture** pattern with clear separation of concerns:

```
Request Flow:
HTTP Request → Handler → Service → DB Layer → Database
                  ↓         ↓          ↓
                 DTOs   Validation   Models
                  ↓         ↓          ↓
HTTP Response ← Handler ← Service ← DB Layer
```

### Layer Responsibilities

| Layer | Directory | Responsibility |
|-------|-----------|----------------|
| **Handlers** | `src/handlers/` | HTTP request/response handling, parameter extraction |
| **Services** | `src/services/` | Business logic, validation rules |
| **DB Layer** | `src/db/` | Database queries and transactions |
| **Models** | `src/models/` | Database entity representations |
| **DTOs** | `src/dto/` | API request/response schemas |
| **Errors** | `src/errors/` | Custom error types and HTTP error responses |
| **Config** | `src/config/` | Application configuration (database, etc.) |
| **Routes** | `src/routes/` | Route registration and middleware configuration |

### Routing Configuration

The application uses **scoped middleware** for optimal path handling:

- **`/api/*` routes**: Protected by `NormalizePath` middleware with `TrailingSlash::Trim`
  - Accepts both `/api/users` and `/api/users/`
  - Normalizes to canonical form without trailing slash
  - Follows REST API best practices

- **`/swagger-ui/*` routes**: No normalization applied
  - Requires trailing slash: `/swagger-ui/`
  - Standard Swagger UI convention

- **Other routes** (e.g., `/health`): Strict matching
  - Only exact paths accepted
  - No trailing slash handling

This configuration provides the best of both worlds: forgiving API routes for clients while maintaining strict conventions where needed.

### Benefits

- **Testability**: Each layer can be unit tested independently
- **Maintainability**: Clear structure makes code easy to navigate
- **Scalability**: Easy to add new features following established patterns
- **Separation of Concerns**: Each layer has a single responsibility

## Development

### Check Code Compilation
```bash
cargo check
```

### Running Tests
```bash
cargo test
```

### Building for Production
```bash
cargo build --release
```

### Running with Logging
```bash
RUST_LOG=debug cargo run
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Troubleshooting

### Database Connection Issues

**Problem**: `Access denied for user 'root'@'192.168.65.1'`

**Solution**: This typically occurs with Docker Desktop MySQL. Try:
1. Update `.env` to use `127.0.0.1` instead of `localhost`:
   ```env
   DATABASE_URL=mysql://root:password@127.0.0.1:3306/rust
   ```
2. For no password, include the colon: `mysql://root:@127.0.0.1:3306/rust`

**Problem**: `Connection refused`

**Solution**: Ensure MySQL is running:
```bash
# Check if MySQL is running
mysql --version
# Or for Docker
docker ps | grep mysql
```

**Problem**: `Unknown database 'rust'`

**Solution**: Create the database first:
```sql
CREATE DATABASE rust;
```

### Port Already in Use

If port 8080 is already in use, change the `PORT` in `.env`:
```env
PORT=8081
```

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Contribution Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Update documentation as needed
- Ensure `cargo fmt` and `cargo clippy` pass

## Best Practices Implemented

- ✅ Layered architecture for maintainability
- ✅ Dependency injection with Actix-web's `Data` extractor
- ✅ Connection pooling for efficient database access
- ✅ Proper error handling with custom error types
- ✅ Input validation in the service layer
- ✅ OpenAPI documentation generated from code
- ✅ Environment-based configuration
- ✅ Structured logging with env_logger
- ✅ Async/await for non-blocking I/O

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Actix-web](https://actix.rs/)
- Inspired by clean architecture principles
- OpenAPI integration via [utoipa](https://github.com/juhaku/utoipa)

---

**Happy Coding!** If you find this project helpful, please consider giving it a ⭐ on GitHub!
