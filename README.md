# Notes API - Rust Backend

A modern REST API built with Rust, Axum, and PostgreSQL that provides a complete notes management system with user authentication.

## Features

- **User Authentication**: JWT-based authentication with registration and login
- **Notes Management**: Full CRUD operations for notes
- **Search Functionality**: Search notes by title and content
- **Database Integration**: Uses PostgreSQL with stored procedures
- **API Documentation**: Auto-generated Swagger UI documentation
- **Input Validation**: Request validation with custom error messages
- **Middleware**: Authentication middleware for protected routes
- **CORS Support**: Cross-origin resource sharing enabled

## Project Structure

```
src/
├── main.rs                    # Application entry point
├── handlers/                  # HTTP request handlers
│   ├── mod.rs
│   ├── auth.rs               # Authentication handlers
│   ├── notes.rs              # Notes handlers
│   └── users.rs              # User handlers
├── models/                   # Data models and schemas
│   ├── mod.rs
│   ├── auth.rs               # Authentication models
│   ├── notes.rs              # Notes models
│   └── users.rs              # User models
├── routes/                   # Route definitions
│   └── mod.rs
├── services/                 # Business logic layer
│   ├── mod.rs
│   ├── database.rs           # Database connection pool
│   ├── auth_service.rs       # Authentication service
│   ├── user_service.rs       # User service
│   └── note_service.rs       # Notes service
└── utils/                    # Utility functions
    ├── mod.rs
    ├── jwt.rs                # JWT token utilities
    └── auth_middleware.rs    # Authentication middleware
```

## Prerequisites

- Rust 1.70 or higher
- PostgreSQL 13 or higher
- A PostgreSQL database named `notesdb`

## Setup Instructions

### 1. Database Setup

First, create the PostgreSQL database and tables:

```sql
-- Connect to PostgreSQL as postgres user
CREATE DATABASE notesdb;
```

Then run the provided SQL scripts:
1. `1_CreateTables.sql` - Creates the Users and Notes tables
2. `2_CreateStoredProcedures.sql` - Creates all the stored procedures

### 2. Environment Configuration

Copy the example environment file and update with your settings:

```bash
cp .env.example .env
```

Edit `.env` with your database credentials:

```env
DATABASE_URL=host=127.0.0.1 port=5432 dbname=notesdb user=postgres password=postgres
JWT_SECRET=your_very_secure_secret_key_change_this_in_production
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=info
```

### 3. Build and Run

```bash
# Install dependencies and build
cargo build

# Run the application
cargo run
```

The server will start on `http://127.0.0.1:3000`

## API Documentation

Once the server is running, you can access the interactive API documentation at:

- **Swagger UI**: `http://127.0.0.1:3000/swagger-ui`
- **OpenAPI JSON**: `http://127.0.0.1:3000/api-docs/openapi.json`

## API Endpoints

### Authentication

- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Login user

### Users (Protected)

- `GET /api/v1/users/{id}` - Get user by ID

### Notes (Protected)

- `POST /api/v1/notes` - Create a new note
- `GET /api/v1/notes` - Get all user notes
- `GET /api/v1/notes/{id}` - Get note by ID
- `PUT /api/v1/notes/{id}` - Update note
- `DELETE /api/v1/notes/{id}` - Delete note
- `GET /api/v1/notes/search?search_term={term}` - Search notes

## Authentication

Protected endpoints require a JWT token in the Authorization header:

```
Authorization: Bearer <your_jwt_token>
```

## Example Usage

### Register a new user

```bash
curl -X POST http://127.0.0.1:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### Login

```bash
curl -X POST http://127.0.0.1:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### Create a note

```bash
curl -X POST http://127.0.0.1:3000/api/v1/notes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your_jwt_token>" \
  -d '{
    "title": "My First Note",
    "content": "This is the content of my first note."
  }'
```

### Get all notes

```bash
curl -X GET http://127.0.0.1:3000/api/v1/notes \
  -H "Authorization: Bearer <your_jwt_token>"
```

### Search notes

```bash
curl -X GET "http://127.0.0.1:3000/api/v1/notes/search?search_term=first" \
  -H "Authorization: Bearer <your_jwt_token>"
```

## Database Schema

The application uses your existing PostgreSQL stored procedures:

- `sp_register_user` - Register new user
- `sp_login_user` - User login
- `sp_get_user_by_id` - Get user by ID
- `sp_create_note` - Create new note
- `sp_get_user_notes` - Get user's notes
- `sp_get_note_by_id` - Get note by ID
- `sp_update_note` - Update note
- `sp_delete_note` - Delete note
- `sp_search_notes` - Search notes

## Security Features

- Password hashing with bcrypt
- JWT tokens with expiration
- Input validation and sanitization
- SQL injection prevention through parameterized queries
- CORS protection
- Authentication middleware for protected routes

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Production Considerations

1. **Environment Variables**: Use strong, unique JWT secrets
2. **Database**: Use connection pooling for better performance
3. **Logging**: Configure appropriate log levels
4. **HTTPS**: Use TLS in production
5. **Rate Limiting**: Implement rate limiting for API endpoints
6. **Monitoring**: Add health check endpoints
7. **Docker**: Consider containerization for deployment

## License

This project is licensed under the MIT License.