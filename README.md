# Customer Support Ticketing System

A comprehensive backend system for managing customer support tickets built with **Rust**, **Axum**, and **PostgreSQL**.

## 🚀 Features

### Core Functionality
- **User Management**: Registration, authentication, and role-based access control
- **Ticket Management**: Create, read, update, and delete support tickets
- **Comment System**: Real-time communication between customers and agents
- **Email Notifications**: Automated email notifications for ticket updates
- **Role-Based Access**: Customer, Agent, and Admin roles with different permissions

### Technical Features
- **JWT Authentication**: Secure token-based authentication
- **Argon2 Password Hashing**: Industry-standard password security
- **PostgreSQL Database**: Robust data persistence with SQLx
- **RESTful API**: Clean, well-structured API endpoints
- **Docker Support**: Containerized deployment ready
- **Email Integration**: SMTP email notifications (Gmail/Resend support)

## 🛠️ Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with SQLx ORM
- **Authentication**: JWT tokens with Argon2 password hashing
- **Email**: Lettre SMTP client
- **Containerization**: Docker & Docker Compose
- **Validation**: Serde for JSON serialization/deserialization

## 📋 Prerequisites

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **PostgreSQL** (version 12 or higher) - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Docker** (optional) - [Install Docker](https://docs.docker.com/get-docker/)

## 🚀 Quick Start

### Option 1: Using Docker (Recommended for new users)

```bash
# 1. Clone the repository
git clone <your-repo-url>
cd ticket_service_backend

# 2. Copy environment file
cp env.example .env

# 3. Start with Docker Compose (this will set up everything automatically)
docker-compose up -d

# 4. Check if everything is running
docker-compose ps

# 5. View logs
docker-compose logs -f
```

The server will be available at `http://127.0.0.1:3000`

### Option 2: Manual Setup

#### 1. Clone the Repository
```bash
git clone <your-repo-url>
cd ticket_service_backend
```

#### 2. Install PostgreSQL

**On macOS (using Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

**On Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**On Windows:**
Download and install from [PostgreSQL official website](https://www.postgresql.org/download/windows/)

#### 3. Set Up Database
```bash
# Create database user (if not exists)
sudo -u postgres createuser --interactive
# Enter your username when prompted

# Create database
createdb support_ticketing_system

# Run migrations
psql -d support_ticketing_system -f migrations/20240101000000_initial_schema.sql
```

#### 4. Configure Environment
```bash
# Copy environment template
cp env.example .env

# Edit .env with your settings
nano .env
```

Update the `.env` file with your database settings:
```env
DATABASE_URL=postgresql://your_username@localhost/support_ticketing_system
JWT_SECRET=your-super-secret-jwt-key-change-this
```

#### 5. Run the Application
```bash
# Install dependencies and run
cargo run
```

The server will start at `http://127.0.0.1:3000`

## 🔧 Troubleshooting

### Common Issues

**1. Database Connection Error**
```bash
# Check if PostgreSQL is running
sudo systemctl status postgresql  # Linux
brew services list | grep postgresql  # macOS

# Test database connection
psql -d support_ticketing_system -c "SELECT 1;"
```

**2. Port Already in Use**
```bash
# Check what's using port 3000
lsof -i :3000

# Kill the process or change port in .env
```

**3. Permission Denied (Database)**
```bash
# Fix PostgreSQL permissions
sudo -u postgres psql
CREATE USER your_username WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE support_ticketing_system TO your_username;
\q
```

**4. Rust Dependencies Issues**
```bash
# Clean and rebuild
cargo clean
cargo build
```

## 📚 API Endpoints

### Authentication
- `POST /register` - Register new user
- `POST /login` - User login

### Users
- `GET /users` - Get all users (Admin/Agent only)
- `GET /users/{id}` - Get user by ID

### Tickets
- `GET /tickets` - Get all tickets
- `POST /tickets` - Create new ticket
- `GET /tickets/{id}` - Get ticket by ID
- `PUT /tickets/{id}` - Update ticket
- `DELETE /tickets/{id}` - Delete ticket

### Comments
- `GET /tickets/{id}/comments` - Get ticket comments
- `POST /tickets/{id}/comments` - Add comment to ticket

### Notifications
- `GET /notifications` - Get user notifications
- `PUT /notifications/{id}/read` - Mark notification as read

## 🧪 Testing the API

### Quick Test with curl
```bash
# Test health endpoint
curl http://127.0.0.1:3000/health

# Register a test user
curl -X POST http://127.0.0.1:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "first_name": "Test",
    "last_name": "User",
    "role": "customer"
  }'

# Login
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'

# Create a ticket (replace YOUR_JWT_TOKEN with token from login)
curl -X POST http://127.0.0.1:3000/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Ticket",
    "description": "This is a test ticket",
    "priority": "medium",
    "customer_id": 1
  }'
```

## 🐳 Docker Deployment

### Using Docker Compose (Recommended)
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild and restart
docker-compose up -d --build
```

### Manual Docker Build
```bash
# Build image
docker build -t support-ticketing-system .

# Run container
docker run -p 3000:3000 --env-file .env support-ticketing-system
```

## 🔒 Security Features

- **JWT Authentication**: Secure token-based sessions
- **Argon2 Password Hashing**: Industry-standard password security
- **Role-Based Access Control**: Different permissions per user role
- **Input Validation**: Comprehensive request validation
- **SQL Injection Protection**: Parameterized queries with SQLx

## 🚀 Production Deployment

### Environment Setup
1. Set up PostgreSQL database
2. Configure environment variables
3. Set up email service (Gmail/Resend)
4. Configure reverse proxy (nginx)

### Security Checklist
- [ ] Change default JWT secret
- [ ] Use HTTPS in production
- [ ] Set up proper firewall rules
- [ ] Configure database backups
- [ ] Set up monitoring and logging

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For support and questions:
- Create an issue in this repository
- Check the API documentation
- Review the database schema

## 🎯 Roadmap

- [ ] WebSocket real-time notifications
- [ ] File attachments for tickets
- [ ] Advanced search and filtering
- [ ] Knowledge base management
- [ ] Reporting and analytics
- [ ] Mobile app support
- [ ] Multi-language support

---

**Built with ❤️ using Rust and Axum** 