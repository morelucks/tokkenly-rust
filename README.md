# Tokkenly Rust Server

A Rust web server with user authentication capabilities built using Actix-web and MySQL.

## Features

- User registration with password hashing
- User login with JWT token authentication
- MySQL database integration
- RESTful API endpoints

## Setup

1. **Database Setup**
   ```bash
   # Make sure MySQL is running
   mysql -u root -p
   # Enter your password (default: 123456)
   
   # Create database
   CREATE DATABASE tokkenlydb;
   USE tokkenlydb;
   
   # Run the setup script
   source database_setup.sql;
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Run the Server**
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8000`

## API Endpoints

### User Registration
- **POST** `/auth/register`
- **Body:**
  ```json
  {
    "full_name": "John Doe",
    "email": "john@example.com",
    "password": "securepassword123"
  }
  ```
- **Response:**
  ```json
  {
    "message": "User registered successfully",
    "user_id": "uuid-here"
  }
  ```

### User Login
- **POST** `/auth/login`
- **Body:**
  ```json
  {
    "email": "john@example.com",
    "password": "securepassword123"
  }
  ```
- **Response:**
  ```json
  {
    "token": "jwt-token-here",
    "user": {
      "user_id": "uuid-here",
      "full_name": "John Doe",
      "email": "john@example.com",
      "password": ""
    }
  }
  ```

### Other Endpoints
- **GET** `/home` - Welcome message
- **GET** `/hello/{full_name}/{email}` - Hello user endpoint
- **POST** `/user/create` - Legacy user creation endpoint

## Security Features

- Passwords are hashed using bcrypt
- JWT tokens for session management
- Email uniqueness validation
- Input validation and error handling

## Database Schema

The `users` table contains:
- `user_id` (UUID) - Primary key
- `full_name` - User's full name
- `email` - Unique email address
- `password_hash` - Bcrypt hashed password
- `created_at` - Account creation timestamp
- `updated_at` - Last update timestamp
