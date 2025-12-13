# Actix-web REST API with MySQL

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-ready REST API template built with Actix-web, MySQL, and OpenAPI documentation. This project demonstrates best practices for building scalable, maintainable web services in Rust.

## Table of Contents

- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [API Documentation](#api-documentation)
- [API Endpoints](#api-endpoints)
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
- ✅ Request validation in service layer
- 🌐 CORS support for cross-origin requests
- 📝 Structured logging middleware
- 🏗️ Clean architecture with clear separation of concerns
- ⚡ Async/await throughout for optimal performance
- 🔧 Environment-based configuration

## Technology Stack

- **[Actix-web](https://actix.rs/)** - Fast, pragmatic web framework for Rust
- **[SQLx](https://github.com/launchbadge/sqlx)** - Async, pure Rust SQL toolkit with compile-time checked queries
- **[utoipa](https://github.com/juhaku/utoipa)** - Auto-generated OpenAPI documentation from code
- **[Serde](https://serde.rs/)** - Serialization/deserialization framework
- **[Tokio](https://tokio.rs/)** - Async runtime for Rust
- **MySQL 8.0+** - Relational database

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

- **Rust** 1.75 or higher ([install here](https://www.rust-lang.org/tools/install))
- **MySQL** 8.0 or higher (or Docker)
- **cargo** (comes with Rust)

## Quick Start

### Option 1: Local MySQL

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

### Option 2: Using Docker for MySQL

1. **Start MySQL with Docker**
   ```bash
   docker run --name mysql-actix -e MYSQL_ROOT_PASSWORD=password -e MYSQL_DATABASE=rust -p 3306:3306 -d mysql:8.0
   ```

2. **Run migrations**
   ```bash
   docker exec -i mysql-actix mysql -uroot -ppassword rust < migrations/01_create_users_table.sql
   ```

3. **Update `.env` and run**
   ```env
   DATABASE_URL=mysql://root:password@127.0.0.1:3306/rust
   ```
   ```bash
   cargo run
   ```

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

### Get All Users
```bash
curl http://127.0.0.1:8080/api/users
```

### Get a User by ID
```bash
curl http://127.0.0.1:8080/api/users/1
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
| **Routes** | `src/routes/` | Route registration and grouping |

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
