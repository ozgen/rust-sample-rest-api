# Rust Sample REST API

A minimal REST API project written in Rust using:
- [`hyper`](https://crates.io/crates/hyper) for HTTP server
- [`sqlx`](https://crates.io/crates/sqlx) for PostgreSQL database integration and migrations
- [`dotenvy`](https://crates.io/crates/dotenvy) for environment variable management

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli)

Install `sqlx-cli`:

```bash
cargo install sqlx-cli --no-default-features --features postgres
````

### Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-sample-rest-api.git
cd rust-sample-rest-api
```

2. Start the PostgreSQL database using Docker Compose:

```bash
docker-compose -f docker/docker-compose.yaml up -d
```

This will start a local PostgreSQL instance accessible at `localhost:54322`.

3. Create a `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:54322/app
```

4. Run the database migration and start the application:

```bash
cargo run
```

### Running the Server

The application will start on:

```
http://127.0.0.1:8080
```

## Project Structure

```
src/
├── config.rs        # Loads environment variables
├── handlers.rs      # HTTP request handlers
├── routes.rs        # Routing logic
└── main.rs          # App entry point
migrations/          # SQL migration files (.up.sql / .down.sql)
docker/
└── docker-compose.yaml # PostgreSQL setup for local development
```

## Environment Variables

| Variable       | Description                             |
| -------------- | --------------------------------------- |
| `DATABASE_URL` | PostgreSQL connection string (required) |


