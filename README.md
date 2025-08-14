# ILM - Rust Migration

This is a Rust migration of the original Go application using Actix-Web framework and clean architecture principles.

## Architecture

The application follows Clean Architecture with the following layers:

### 1. Domain Layer (`src/domain/`)
- **Entities**: Core business objects (Project)
- **Errors**: Application-specific error types
- **Traits**: Repository interfaces

### 2. Application Layer (`src/application/`)
- **Services**: Business logic and use cases
- **Repository Traits**: Interfaces for data access

### 3. Infrastructure Layer (`src/infrastructure/`)
- **Database**: Database connection and setup
- **Repository Implementations**: Concrete implementations of repository traits

### 4. Presentation Layer (`src/presentation/`)
- **Handlers**: HTTP request handlers
- **DTOs**: Data transfer objects for API

## Dependencies

- **actix-web**: Web framework
- **sqlx**: Async SQL toolkit
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **uuid**: UUID generation and parsing
- **chrono**: Date and time handling
- **config**: Configuration management
- **tracing**: Structured logging

## Configuration

Configuration is loaded from `config/config.toml`:

```toml
[database]
name = "ilm"
user = "ilm_user"
password = "ilm_password"
host = "localhost"
port = 5432

[server]
host = "127.0.0.1"
port = 10001
```

Environment-specific configs can be created as `config/config.{env}.toml`.

## Running the Application

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Set up the database** (PostgreSQL):
   ```bash
   # Using docker-compose (if available)
   docker-compose up -d
   
   # Or manually create database
   createdb ilm
   ```

3. **Run migrations**:
   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

   Or for development with auto-reload:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

## API Endpoints

### Get Project
- **URL**: `GET /api/v1/projects?id={uuid}`
- **Description**: Retrieve a project by ID
- **Response**: 
  ```json
  {
    "id": "uuid",
    "name": "Project Name",
    "description": "Project Description",
    "flow_stages_id": "stage_id",
    "created_at": "2023-01-01T00:00:00Z",
    "created_by": "user_id",
    "updated_at": "2023-01-01T00:00:00Z"
  }
  ```

## Key Changes from Go Version

1. **Type Safety**: Rust's type system provides compile-time guarantees
2. **Error Handling**: Using `Result<T, E>` for explicit error handling
3. **Async/Await**: Native async support with tokio runtime
4. **Memory Safety**: No garbage collector, zero-cost abstractions
5. **UUID**: Using proper UUID type instead of strings
6. **Timestamps**: Using `chrono::DateTime<Utc>` instead of integers

## Testing

Run tests with:
```bash
cargo test
```

For integration tests with database:
```bash
# Set test database URL
export DATABASE_URL="postgres://ilm_user:ilm_password@localhost:5432/ilm_test"
cargo test
```

## Building for Production

```bash
# Build optimized release
cargo build --release

# The binary will be in target/release/ilm
./target/release/ilm
```

## Docker

Build and run with Docker:
```bash
# Build image
docker build -t ilm-rust .

# Run container
docker run -p 10001:10001 ilm-rust
```

## Migration Notes

### Database Schema Changes
- `id` field changed from `VARCHAR(255)` to `UUID`
- Timestamp fields changed from `INTEGER` to `TIMESTAMPTZ`
- Added proper constraints and defaults

### Architectural Improvements
- Stricter separation of concerns
- Dependency injection using trait objects
- Better error handling with custom error types
- Structured logging with tracing

### Performance Benefits
- Zero-cost abstractions
- No garbage collection pauses
- Efficient async I/O with tokio
- Compiled binary for faster startup
