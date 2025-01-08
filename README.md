
# Jordanopotato Rust Project

This Rust project provides a web server (using **[Axum](https://crates.io/crates/axum)**) to manage users, conversation summaries, and chat functionality with OpenAI. It leverages **[sqlx](https://crates.io/crates/sqlx)** for PostgreSQL database operations and runs migrations on startup. The application is fully asynchronous, powered by **[tokio](https://crates.io/crates/tokio)**.

## Table of Contents

1. [Project Structure](#project-structure)
2. [Getting Started](#getting-started)
3. [Environment Variables](#environment-variables)
4. [Running the Application](#running-the-application)
5. [Available Endpoints](#available-endpoints)
6. [Testing](#testing)

---

## Project Structure

```
.
├── Cargo.toml
├── src
│   ├── db.rs              # Database connection pooling, migrations, CRUD logic
│   ├── lib.rs             # Library module definitions
│   ├── main.rs            # Application entrypoint; sets up Axum server
│   ├── models.rs          # Models (User, Conversation)
│   └── routes.rs          # Axum routes for login, chat, prompt, summary
├── tests
│   ├── db_tests.rs        # Integration tests for DB functions
│   └── server_tests.rs    # Basic server endpoint tests (e.g., root, health)
└── ...
```

### `src/main.rs`
- **Entrypoint** for the application.
- Loads environment variables from `.env` (using **dotenv**).
- Initializes a **Postgres** pool (with `db::init_db_pool`).
- Automatically runs migrations (`db::run_migrations`).
- Builds the Axum router from `routes::create_router` and starts an HTTP server on `0.0.0.0:3000`.

### `src/db.rs`
- Functions for:
  - **Postgres** connection pooling (`init_db_pool`).
  - Basic migrations (`run_migrations`) that ensure `users` and `conversations` tables exist.
  - CRUD operations: Create or retrieve users, update prompts, manage conversation summaries, etc.

### `src/models.rs`
- **User** and **Conversation** structs, each with `Deserialize`, `Serialize`, and `sqlx::FromRow` for DB integration.

### `src/routes.rs`
- Houses **Axum** route handlers and the `create_router` function:
  - **`POST /login`**: Creates or retrieves a user, returning their ID and any existing conversation summary.
  - **`POST /chat`**: Main chat endpoint (integrates with OpenAI GPT-4).
  - **`GET /prompt/:user_id`** and **`POST /prompt/:user_id`**: For retrieving/updating user’s prompt.
  - **`GET /summary/:user_id`**: Retrieves a user’s conversation summary.

### `tests/db_tests.rs`
- Integration tests for database logic:
  - Creates a **test** database (using `TEST_DATABASE_URL`), drops tables, runs migrations, then validates DB operations.

### `tests/server_tests.rs`
- Basic **Axum** server tests for routes such as `/` (root) and `/health`, ensuring they return expected status codes or bodies.

---

## Getting Started

1. **Clone** the repository:

   ```bash
   git clone https://github.com/your-username/your-repo-name.git
   cd your-repo-name
   ```

2. **Install Rust** (if not already done):
   [Rust installation instructions](https://www.rust-lang.org/tools/install)

3. **Create a `.env` file** in the project root. See [Environment Variables](#environment-variables) for details.

4. **Set up PostgreSQL** and ensure your database URL is correct.

5. **(Optional)**: If you want to run the tests, ensure you also have a separate test database available.

---

## Environment Variables

The project relies on these environment variables in a `.env` file:

```bash
# Main DB URL
DATABASE_URL=postgres://postgres:password@localhost:5432/my_database

# Your OpenAI API key
MY_OPENAI_KEY=sk-123yourOpenAIKey

# For testing (if you run cargo test)
TEST_DATABASE_URL=postgres://postgres:password@localhost:5432/my_database_test
```

> **Important**:
> - The **tests** will use `TEST_DATABASE_URL`, dropping and recreating tables, so do **not** point this to your production database.
> - **MY_OPENAI_KEY** is needed for the `/chat` endpoint to function properly.

---

## Running the Application

1. Make sure `.env` contains valid `DATABASE_URL` and `MY_OPENAI_KEY`.
2. Run:

   ```bash
   cargo run
   ```

This will:

1. Load environment variables.
2. Initialize and test the DB connection.
3. Run migrations to ensure `users` and `conversations` tables exist.
4. Start an Axum server on `http://0.0.0.0:3000`.

You should see logs like:

```
Database connected and migrations applied successfully!
Server listening on http://0.0.0.0:3000
```

---

## Available Endpoints

Below is a quick reference to the primary routes. All return JSON unless stated otherwise:

1. **POST** `/login`
   **Request Body**:
   ```json
   {
     "username": "someusername"
   }
   ```
   **Response**:
   ```json
   {
     "user_id": 123,
     "summary": "Possible existing conversation summary"
   }
   ```
   - Creates or retrieves a user and any existing conversation summary.

2. **POST** `/chat`
   **Request Body**:
   ```json
   {
     "user_id": 123,
     "message": "Hello, I'd like to talk about..."
   }
   ```
   **Response**:
   ```json
   {
     "reply": "AI-generated response...",
     "user_id": 123
   }
   ```
   - Integrates with OpenAI to generate a reply and updates the conversation summary in the DB.

3. **GET** `/prompt/:user_id`
   - Returns the user’s custom (or default) prompt as **plain text**.
   - `404` if the user doesn’t exist.

4. **POST** `/prompt/:user_id`
   **Request Body**:
   ```json
   {
     "new_prompt": "My custom prompt content"
   }
   ```
   **Response**:
   ```json
   {
     "prompt": "My custom prompt content"
   }
   ```
   - Updates the user’s custom prompt.
   - `404` if the user doesn’t exist.

5. **GET** `/summary/:user_id`
   **Response**:
   ```json
   {
     "summary": "Current conversation summary"
   }
   ```
   - Returns the conversation summary or `404` if missing.

---

## Testing

1. Make sure `.env` has a `TEST_DATABASE_URL`.
2. Run:

   ```bash
   cargo test -- --test-threads=1
   ```

   Why `--test-threads=1`? Because we drop/recreate tables in tests, so running in parallel can cause conflicts unless carefully managed.

### What happens during tests?
- The **`db_tests.rs`** suite:
  1. Connects to your **test** database.
  2. Drops existing tables (`users`, `conversations`).
  3. Runs migrations.
  4. Exercises DB functions (e.g., `create_user`, `update_user_prompt`, etc.).
  5. Asserts correctness (e.g., verifying prompts or conversation summaries).

- The **`server_tests.rs`** suite:
  - Launches a mock router with minimal routes (`/`, `/health`) to ensure these endpoints respond correctly (e.g., `200 OK`).

---

**Enjoy exploring, extending, and testing your Jordanopotato Rust Project!** If you have any issues or suggestions, feel free to open an issue or submit a pull request.
