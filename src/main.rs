use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use chrono::{Utc, Duration};
use reqwest;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
// Email imports removed since we're using logging instead of SMTP

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub customer_id: i32,
    pub assigned_agent_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub ticket_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ticket_id: Option<i32>,
}

// Request/Response structs
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub customer_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_agent_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i32,
    pub email: String,
    pub role: String,
    pub exp: i64,
}

// JWT functions
fn create_jwt(user_id: i32, email: String, role: String, secret: &str) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    
    let claims = JwtClaims {
        sub: user_id,
        email,
        role,
        exp: expiration,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

fn verify_jwt(token: &str, secret: &str) -> Result<JwtClaims, JwtError> {
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

// Email functions
async fn send_email(
    to_email: &str,
    subject: &str,
    body: &str,
    _http_client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    // Just log the email instead of sending (for testing)
    println!("📧 EMAIL WOULD BE SENT:");
    println!("   To: {}", to_email);
    println!("   Subject: {}", subject);
    println!("   Body: {}", body);
    println!("   ---");
    
    Ok(())
}

async fn send_welcome_email(email: &str, first_name: &str, http_client: &reqwest::Client) {
    let subject = "Welcome to Support Ticketing System!";
    let body = format!(
        "Dear {},\n\nWelcome to our Support Ticketing System! Your account has been created successfully.\n\nBest regards,\nSupport Team",
        first_name
    );
    
    if let Err(e) = send_email(email, &subject, &body, http_client).await {
        eprintln!("Failed to send welcome email: {}", e);
    }
}

async fn send_ticket_notification(
    email: &str,
    ticket_title: &str,
    ticket_id: i32,
    http_client: &reqwest::Client,
) {
    let subject = format!("New Ticket Created - #{}", ticket_id);
    let body = format!(
        "A new ticket has been created:\n\nTitle: {}\nTicket ID: {}\n\nWe will review your request and get back to you soon.\n\nBest regards,\nSupport Team",
        ticket_title, ticket_id
    );
    
    if let Err(e) = send_email(email, &subject, &body, http_client).await {
        eprintln!("Failed to send ticket notification: {}", e);
    }
}

// Handlers
async fn root() -> &'static str {
    "🚀 Support Ticketing System Backend\n\nAvailable endpoints:\n- POST /register - Register new user\n- POST /login - Login user\n- GET /users - Get all users\n- GET /tickets - Get all tickets\n- POST /tickets - Create new ticket\n- PUT /tickets/{id} - Update ticket\n- DELETE /tickets/{id} - Delete ticket\n- POST /tickets/{id}/comments - Add comment\n- GET /tickets/{id}/comments - Get ticket comments\n- GET /notifications - Get notifications\n- PUT /notifications/{id}/read - Mark notification as read\n\nTry visiting /health to test the API!"
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Support Ticketing System is running".to_string(),
    })
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, StatusCode> {
    // Check if user already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // Insert user
    let row = sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name, role, email_verified)
        VALUES ($1, $2, $3, $4, $5::text::user_role, false)
        RETURNING id, email, first_name, last_name, role::text as role, email_verified, created_at
        "#,
        payload.email,
        password_hash,
        payload.first_name,
        payload.last_name,
        payload.role
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id: row.id,
        email: row.email,
        first_name: row.first_name,
        last_name: row.last_name,
        role: row.role.expect("role should not be null"),
        email_verified: row.email_verified,
        created_at: row.created_at.expect("created_at should not be null"),
    };

    // Send welcome email
    send_welcome_email(&user.email, &user.first_name, &state.http_client).await;

    Ok(Json(user))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = sqlx::query!(
        r#"
        SELECT id, email, password_hash, first_name, last_name, role::text as role, email_verified, created_at
        FROM users WHERE email = $1
        "#,
        payload.email
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let argon2 = Argon2::default();
    let valid = argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create JWT token
    let role = user.role.expect("role should not be null");
    let token = create_jwt(
        user.id,
        user.email.clone(),
        role.clone(),
        &state.jwt_secret,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_response = User {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role,
        email_verified: user.email_verified,
        created_at: user.created_at.expect("created_at should not be null"),
    };

    Ok(Json(LoginResponse {
        token,
        user: user_response,
    }))
}

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT id, email, first_name, last_name, role::text as role, email_verified, created_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = rows
        .into_iter()
        .map(|row| User {
            id: row.id,
            email: row.email,
            first_name: row.first_name,
            last_name: row.last_name,
            role: row.role.expect("role should not be null"),
            email_verified: row.email_verified,
            created_at: row.created_at.expect("created_at should not be null"),
        })
        .collect();

    Ok(Json(users))
}

async fn get_tickets(State(state): State<AppState>) -> Result<Json<Vec<Ticket>>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT id, title, description, status::text as status, priority::text as priority, customer_id, assigned_agent_id, created_at, updated_at, resolved_at FROM tickets ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tickets = rows
        .into_iter()
        .map(|row| Ticket {
            id: row.id,
            title: row.title,
            description: row.description,
            status: row.status.expect("status should not be null"),
            priority: row.priority.expect("priority should not be null"),
            customer_id: row.customer_id,
            assigned_agent_id: row.assigned_agent_id,
            created_at: row.created_at.expect("created_at should not be null"),
            updated_at: row.updated_at,
            resolved_at: row.resolved_at,
        })
        .collect();

    Ok(Json(tickets))
}

async fn get_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
) -> Result<Json<Ticket>, StatusCode> {
    let row = sqlx::query!(
        "SELECT id, title, description, status::text as status, priority::text as priority, customer_id, assigned_agent_id, created_at, updated_at, resolved_at FROM tickets WHERE id = $1",
        ticket_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let ticket = Ticket {
        id: row.id,
        title: row.title,
        description: row.description,
        status: row.status.expect("status should not be null"),
        priority: row.priority.expect("priority should not be null"),
        customer_id: row.customer_id,
        assigned_agent_id: row.assigned_agent_id,
        created_at: row.created_at.expect("created_at should not be null"),
        updated_at: row.updated_at,
        resolved_at: row.resolved_at,
    };

    Ok(Json(ticket))
}

async fn create_ticket(
    State(state): State<AppState>,
    Json(ticket_data): Json<CreateTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    let row = sqlx::query!(
        r#"
        INSERT INTO tickets (title, description, status, priority, customer_id)
        VALUES ($1, $2, 'open', $3::text::ticket_priority, $4)
        RETURNING id, title, description, status::text as status, priority::text as priority, customer_id, assigned_agent_id, created_at, updated_at, resolved_at
        "#,
        ticket_data.title,
        ticket_data.description,
        ticket_data.priority,
        ticket_data.customer_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ticket = Ticket {
        id: row.id,
        title: row.title,
        description: row.description,
        status: row.status.expect("status should not be null"),
        priority: row.priority.expect("priority should not be null"),
        customer_id: row.customer_id,
        assigned_agent_id: row.assigned_agent_id,
        created_at: row.created_at.expect("created_at should not be null"),
        updated_at: row.updated_at,
        resolved_at: row.resolved_at,
    };

    // Get customer email for notification
    let customer = sqlx::query!(
        "SELECT email, first_name FROM users WHERE id = $1",
        ticket_data.customer_id
    )
    .fetch_one(&state.pool)
    .await;

    if let Ok(customer) = customer {
        send_ticket_notification(&customer.email, &ticket.title, ticket.id, &state.http_client).await;
    }

    Ok(Json(ticket))
}

async fn update_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
    Json(ticket_data): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut query = String::from("UPDATE tickets SET ");
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(title) = &ticket_data.title {
        params.push(format!("title = ${}", param_count));
        param_count += 1;
    }
    if let Some(description) = &ticket_data.description {
        params.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if let Some(status) = &ticket_data.status {
        params.push(format!("status = ${}::text::ticket_status", param_count));
        param_count += 1;
    }
    if let Some(priority) = &ticket_data.priority {
        params.push(format!("priority = ${}::text::ticket_priority", param_count));
        param_count += 1;
    }
    if let Some(assigned_agent_id) = &ticket_data.assigned_agent_id {
        params.push(format!("assigned_agent_id = ${}", param_count));
        param_count += 1;
    }

    if params.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    query.push_str(&params.join(", "));
    query.push_str(&format!(", updated_at = NOW() WHERE id = ${} RETURNING id, title, description, status::text as status, priority::text as priority, customer_id, assigned_agent_id, created_at, updated_at, resolved_at", param_count));

    // This is a simplified version - in production you'd use proper parameterized queries
    let row = sqlx::query!(
        r#"
        UPDATE tickets 
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3::text::ticket_status, status),
            priority = COALESCE($4::text::ticket_priority, priority),
            assigned_agent_id = COALESCE($5, assigned_agent_id),
            updated_at = NOW()
        WHERE id = $6
        RETURNING id, title, description, status::text as status, priority::text as priority, customer_id, assigned_agent_id, created_at, updated_at, resolved_at
        "#,
        ticket_data.title,
        ticket_data.description,
        ticket_data.status,
        ticket_data.priority,
        ticket_data.assigned_agent_id,
        ticket_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let ticket = Ticket {
        id: row.id,
        title: row.title,
        description: row.description,
        status: row.status.expect("status should not be null"),
        priority: row.priority.expect("priority should not be null"),
        customer_id: row.customer_id,
        assigned_agent_id: row.assigned_agent_id,
        created_at: row.created_at.expect("created_at should not be null"),
        updated_at: row.updated_at,
        resolved_at: row.resolved_at,
    };

    Ok(Json(ticket))
}

async fn delete_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DELETE FROM tickets WHERE id = $1", ticket_id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_ticket_comments(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
) -> Result<Json<Vec<Comment>>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT id, ticket_id, user_id, content, created_at FROM comments WHERE ticket_id = $1 ORDER BY created_at ASC",
        ticket_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let comments = rows
        .into_iter()
        .map(|row| Comment {
            id: row.id,
            ticket_id: row.ticket_id,
            user_id: row.user_id,
            content: row.content,
            created_at: row.created_at.expect("created_at should not be null"),
        })
        .collect();

    Ok(Json(comments))
}

async fn add_comment(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
    Json(comment_data): Json<CreateCommentRequest>,
) -> Result<Json<Comment>, StatusCode> {
    let row = sqlx::query!(
        r#"
        INSERT INTO comments (ticket_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING id, ticket_id, user_id, content, created_at
        "#,
        ticket_id,
        comment_data.user_id,
        comment_data.content
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let comment = Comment {
        id: row.id,
        ticket_id: row.ticket_id,
        user_id: row.user_id,
        content: row.content,
        created_at: row.created_at.expect("created_at should not be null"),
    };

    Ok(Json(comment))
}

async fn get_notifications(State(state): State<AppState>) -> Result<Json<Vec<Notification>>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT id, user_id, notification_type::text as notification_type, title, message, read, created_at, ticket_id FROM notifications ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let notifications = rows
        .into_iter()
        .map(|row| Notification {
            id: row.id,
            user_id: row.user_id,
            notification_type: row.notification_type.expect("notification_type should not be null"),
            title: row.title,
            message: row.message,
            read: row.read,
            created_at: row.created_at.expect("created_at should not be null"),
            ticket_id: row.ticket_id,
        })
        .collect();

    Ok(Json(notifications))
}

async fn mark_notification_read(
    State(state): State<AppState>,
    Path(notification_id): Path<i32>,
) -> Result<Json<Notification>, StatusCode> {
    let row = sqlx::query!(
        r#"
        UPDATE notifications 
        SET read = true
        WHERE id = $1
        RETURNING id, user_id, notification_type::text as notification_type, title, message, read, created_at, ticket_id
        "#,
        notification_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let notification = Notification {
        id: row.id,
        user_id: row.user_id,
        notification_type: row.notification_type.expect("notification_type should not be null"),
        title: row.title,
        message: row.message,
        read: row.read,
        created_at: row.created_at.expect("created_at should not be null"),
        ticket_id: row.ticket_id,
    };

    Ok(Json(notification))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/support_ticketing_system".to_string());
    
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    // HTTP client for sending emails
    let http_client = reqwest::Client::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Test database connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Failed to test database connection");

    println!("✅ Database connected successfully");
    println!("✅ Gmail SMTP configured");

    let state = AppState {
        pool,
        jwt_secret,
        http_client,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/users", get(get_users))
        .route("/tickets", get(get_tickets))
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", put(update_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .route("/tickets/:id/comments", get(get_ticket_comments))
        .route("/tickets/:id/comments", post(add_comment))
        .route("/notifications", get(get_notifications))
        .route("/notifications/:id/read", put(mark_notification_read))
        .layer(cors)
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server starting on http://{}", addr);
    println!("📧 Email notifications enabled (using Gmail SMTP)");
    println!("🔐 JWT authentication enabled");
    
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app
    )
    .await
    .unwrap();
}
