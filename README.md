# Support Ticketing System Backend

A comprehensive Customer Support Ticketing System built with Rust, Axum, and PostgreSQL. This system provides a robust backend for managing customer support operations with real-time collaboration features.

## Features

### Core Features
- **User Management**: Role-based access control (Admin, Agent, Customer)
- **Ticket Management**: Full CRUD operations with status and priority management
- **Comment System**: Public and private comments on tickets
- **Email Integration**: Automated email notifications using templates
- **Real-time Notifications**: WebSocket support for live updates
- **Knowledge Base**: Searchable articles and documentation
- **Advanced Search**: Full-text search across tickets and articles
- **Audit Trail**: Complete history tracking for tickets

### Technical Features
- **JWT Authentication**: Secure token-based authentication
- **Role-based Authorization**: Granular permissions system
- **Database Migrations**: Automated schema management
- **Email Templates**: Configurable email notifications
- **Full-text Search**: PostgreSQL-based search functionality
- **WebSocket Support**: Real-time communication
- **API Documentation**: Comprehensive REST API
- **Error Handling**: Robust error management
- **Logging**: Structured logging with tracing

## Tech Stack

- **Framework**: Axum (Rust web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt password hashing
- **Email**: Lettre SMTP client
- **Search**: PostgreSQL full-text search
- **Real-time**: WebSocket with tokio-tungstenite
- **Validation**: Validator crate
- **Serialization**: Serde
- **Logging**: Tracing

## Prerequisites

- Rust 1.70+ (latest stable)
- PostgreSQL 12+
- SMTP server (Gmail, SendGrid, etc.)

## Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd support_ticketing_system
   ```

2. **Set up PostgreSQL database**
   ```bash
   # Create database
   createdb support_tickets
   
   # Or using psql
   psql -c "CREATE DATABASE support_tickets;"
   ```

3. **Configure environment variables**
   ```bash
   cp env.example .env
   # Edit .env with your configuration
   ```

4. **Install dependencies and run migrations**
   ```bash
   cargo build
   cargo run
   ```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | 3000 |
| `DATABASE_URL` | PostgreSQL connection string | - |
| `JWT_SECRET` | Secret key for JWT tokens | - |
| `JWT_EXPIRATION` | JWT token expiration (seconds) | 86400 |
| `EMAIL_SMTP_HOST` | SMTP server host | - |
| `EMAIL_SMTP_PORT` | SMTP server port | 587 |
| `EMAIL_USERNAME` | SMTP username | - |
| `EMAIL_PASSWORD` | SMTP password | - |
| `EMAIL_FROM` | From email address | - |
| `CORS_ORIGIN` | CORS allowed origins | * |
| `RUST_LOG` | Logging level | info |

## API Documentation

### Authentication

#### Register User
```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123",
  "first_name": "John",
  "last_name": "Doe"
}
```

#### Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

### Tickets

#### Create Ticket
```http
POST /tickets
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Login Issue",
  "description": "Cannot login to the application",
  "priority": "high",
  "category": "authentication",
  "tags": ["login", "urgent"]
}
```

#### Get Tickets
```http
GET /tickets?status=open&priority=high&limit=20&offset=0
Authorization: Bearer <token>
```

#### Update Ticket
```http
PUT /tickets/{ticket_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "status": "in_progress",
  "priority": "medium"
}
```

### Comments

#### Add Comment
```http
POST /tickets/{ticket_id}/comments
Authorization: Bearer <token>
Content-Type: application/json

{
  "content": "Working on this issue",
  "comment_type": "public"
}
```

### Knowledge Base

#### Create Article
```http
POST /knowledge-base
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "How to Reset Password",
  "content": "Step-by-step guide...",
  "category": "account",
  "tags": ["password", "reset"],
  "is_published": true
}
```

#### Search Articles
```http
GET /knowledge-base/search?query=password&category=account
Authorization: Bearer <token>
```

### Search

#### Global Search
```http
GET /search/global?query=login
Authorization: Bearer <token>
```

## Database Schema

### Core Tables

- **users**: User accounts and roles
- **tickets**: Support tickets with status and priority
- **comments**: Public and private comments on tickets
- **notifications**: User notifications
- **knowledge_base_articles**: Knowledge base content
- **email_templates**: Email notification templates
- **ticket_history**: Audit trail for ticket changes

### Key Features

- **UUID Primary Keys**: All entities use UUIDs
- **Timestamps**: Created/updated timestamps on all tables
- **Full-text Search**: PostgreSQL full-text search indexes
- **Foreign Key Constraints**: Referential integrity
- **Triggers**: Automatic updated_at timestamps

## Development

### Running the Application

```bash
# Development mode
cargo run

# Production build
cargo build --release
./target/release/support_ticketing_system
```

### Running Tests

```bash
cargo test
```

### Database Migrations

```bash
# Run migrations
cargo run --bin migrate

# Create new migration
cargo run --bin migrate create <migration_name>
```

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── config/                 # Configuration management
├── database/               # Database connection and migrations
├── models/                 # Data models and DTOs
├── handlers/               # HTTP request handlers
├── services/               # Business logic services
├── middleware/             # Authentication and other middleware
├── email/                  # Email service
├── search/                 # Search functionality
├── websocket/              # Real-time features
└── utils/                  # Utility functions
```

## Security Features

- **Password Hashing**: bcrypt with salt
- **JWT Tokens**: Secure authentication
- **Role-based Access**: Granular permissions
- **Input Validation**: Request validation
- **SQL Injection Protection**: Parameterized queries
- **CORS Configuration**: Cross-origin resource sharing
- **Rate Limiting**: API rate limiting (configurable)

## Monitoring and Logging

- **Structured Logging**: JSON-formatted logs
- **Request Tracing**: Request/response logging
- **Error Tracking**: Comprehensive error handling
- **Health Checks**: `/health` endpoint
- **Metrics**: Performance monitoring

## Deployment

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/support_ticketing_system /usr/local/bin/
CMD ["support_ticketing_system"]
```

### Environment Setup

1. Set up PostgreSQL database
2. Configure environment variables
3. Run database migrations
4. Start the application

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support and questions, please open an issue in the repository or contact the development team. 