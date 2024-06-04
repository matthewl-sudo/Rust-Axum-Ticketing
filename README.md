# Rust Axum Ticketing Management System

#### Table of Contents

- [Description](#description)
- [Features](#features)
- [Installation](#installation)
- [API Endpoints](#api-endpoints)
- [Acknowledgements](#acknowledgements)

### Description

A simple and efficient API built with Rust and Axum. This project provides a set of endpoints for managing a collection of tickets, demonstrating how to build a RESTful API with Axum. This was previously build in a express app but after adopted Axum Rust for the type safety, the challenge and fast performance.

### Features

Highlight the main features of your API.

- [x] RESTful API endpoints for CRUD operations
- [x] JWT & Role-base authentication
- [x] Input validation
- [x] Error handling
- [x] Middleware support
- Assign Tickets to agents
- Add comments and tags
- Create Filters to view specific Tickets
- Show Backlog priority and analytics

---

### Installation

1. **Clone the repository:**

```sh
git clone https://github.com/matthewl-sudo/axum-ticketing.git
```

2.  **Navigate to the project directory:**

```sh
  cd your-repo
```

3. **Install dependencies:**

Ensure you have Rust installed. Then, run:

```sh
  cargo build
```

4. **Set up environment variables:**

Create a .env file and add necessary environment variables.

```sh
  HOST=localhost
  MYSQL_DATABASE=DBname
  MYSQL_USER=root
  MYSQL_PASSWORD=psswd
  PORT=3306
  # Don't worry about the placeholders. Rust supports this feature.
  DATABASE_URL=mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@${HOST}:${PORT}/${MYSQL_DATABASE}

  # Whatever your frontend origin is
  ALLOW_ORIGIN="http://localhost:5173"

  JWT_SECRET=1150950009-8138575101-6639035100
  JWT_EXPIRED_IN=60m
  JWT_MAXAGE=60
```

5. **Run the server:**

```bash
  cargo run
```

The API will be running at http://localhost:3000.

---

### API Endpoints

List and describe the available API endpoints.

#### Authentication

- **POST /login**: Authenticate and receive a JWT token.
  - Request: `{ "email": "your_email", "password": "your_password" }`
  - Response: `{ "token": "your_jwt_token" }`

#### Tickets

- **GET /api/ticket/all**: Retrieve a list of service tickets.
- **POST /api/ticket/**: Create a new service ticket.
  - Request: `{ "Summary": "ticket_summary", "Priority": "ticket_priority" }`
- **GET /api/ticket/:id**: Retrieve a specific Ticket by ID.
- **PATCH /api/ticket/:id**: Update a specific Ticket by ID.
  - Request: `{ "summary": "ticket_summary", "priority": "ticket_priority", "status": "ticket_status" }`
- **DELETE /api/ticket/:id**: Delete a specific ticket by ID.

---

### Acknowledgements

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Rust Lang](https://www.rust-lang.org/)
