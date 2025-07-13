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

- Rust (latest stable version)
- PostgreSQL
- Docker (optional)

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone <your-repo-url>
cd major_rust_projectc
```

### 2. Set Up Database
```bash
# Create PostgreSQL database
psql -h localhost -U postgres -c "CREATE DATABASE support_ticketing_system;"

# Run migrations
psql -h localhost -U postgres -d support_ticketing_system -f migrations/20240101000000_initial_schema.sql
```

### 3. Configure Environment
```bash
# Copy environment template
cp env.example .env

# Edit .env with your database and email settings
DATABASE_URL=postgresql://localhost/support_ticketing_system
JWT_SECRET=your-super-secret-jwt-key
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

### 4. Run the Application
```bash
# Set database URL and run
export DATABASE_URL="postgresql://localhost/support_ticketing_system"
cargo run
```

The server will start at `http://127.0.0.1:3000`

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
- `PATCH /tickets/{id}` - Update ticket
- `DELETE /tickets/{id}` - Delete ticket

### Comments
- `GET /tickets/{id}/comments` - Get ticket comments
- `POST /tickets/{id}/comments` - Add comment to ticket

### Notifications
- `GET /notifications` - Get user notifications

## 🔧 Configuration

### Environment Variables
```env
DATABASE_URL=postgresql://localhost/support_ticketing_system
JWT_SECRET=your-super-secret-jwt-key
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
```

### Email Setup
For Gmail:
1. Enable 2-factor authentication
2. Generate an App Password
3. Use the App Password in SMTP_PASSWORD

For Resend (recommended):
1. Sign up at [resend.com](https://resend.com)
2. Get your API key
3. Use Resend SMTP settings

## 🐳 Docker Deployment

### Using Docker Compose
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Manual Docker Build
```bash
# Build image
docker build -t support-ticketing-system .

# Run container
docker run -p 3000:3000 --env-file .env support-ticketing-system
```

## 📊 Database Schema

### Tables
- **users**: User accounts and authentication
- **tickets**: Support tickets with status and priority
- **comments**: Ticket comments and communication
- **notifications**: User notifications
- **knowledge_base**: Support articles (future feature)

### Enums
- **user_role**: customer, agent, admin
- **ticket_status**: open, pending, closed
- **ticket_priority**: low, medium, high

## 🔒 Security Features

- **JWT Authentication**: Secure token-based sessions
- **Argon2 Password Hashing**: Industry-standard password security
- **Role-Based Access Control**: Different permissions per user role
- **Input Validation**: Comprehensive request validation
- **SQL Injection Protection**: Parameterized queries with SQLx

## 🧪 Testing

### Manual Testing with curl
```bash
# Register user
curl -X POST http://127.0.0.1:3000/register \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123", "first_name": "Test", "last_name": "User", "role": "customer"}'

# Login
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'

# Create ticket (use token from login)
curl -X POST http://127.0.0.1:3000/tickets \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"title": "Test Ticket", "description": "Test description", "priority": "medium", "customer_id": 1}'
```

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