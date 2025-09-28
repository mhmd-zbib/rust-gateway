# API Gateway

A high-performance, secure API gateway built in Rust. Proxies requests to backend services with load balancing, rate limiting, authentication, and monitoring.

## Features

- **Proxying**: Routes HTTP requests to registered backend services.
- **Load Balancing**: Round-robin with circuit breaker for fault tolerance.
- **Rate Limiting**: Global rate limiting (configurable).
- **Authentication**: JWT-based auth for protected routes.
- **Metrics**: Request tracking (total, success, failure).
- **Service Registry**: In-memory registration of backends.
- **Health Checks**: Basic `/health` endpoint.

## Quick Start

1. **Install Rust**: [rustup.rs](https://rustup.rs)
2. **Clone & Run**:
   ```bash
   git clone <repo>
   cd api-gateway
   cargo run
   ```
3. **Server**: Runs on `http://localhost:3000`

## Configuration

Edit `config.yaml`:

```yaml
port: 3000
auth:
  jwt_secret: "your-secret-key"
routes:
  - path: "/api/users"
    service: "users-service"
```

## API Endpoints

- `GET /health` - Health check
- `GET /metrics` - JSON metrics
- `POST /register` - Register backend: `{"service": "name", "url": "http://backend"}`
- `/*` - Proxied routes (require JWT auth)

## Usage

1. **Register Service**:

   ```bash
   curl -X POST http://localhost:3000/register \
     -H "Content-Type: application/json" \
     -d '{"service": "users", "url": "http://localhost:5000"}'
   ```

2. **Proxy Request** (with JWT):
   ```bash
   curl -H "Authorization: Bearer <jwt-token>" \
     http://localhost:3000/api/users
   ```

## Architecture

- **Modular**: Clean separation (handlers, load_balancer, metrics, etc.)
- **Thread-Safe**: Uses `Arc<Mutex<>>` for concurrency.
- **Extensible**: Easy to add new features (e.g., caching, logging).

## Contributing

- Follow Rust best practices.
- Run `cargo clippy` and `cargo fmt`.
- Add tests for new features.

Built with Axum, Reqwest, and Serde.
