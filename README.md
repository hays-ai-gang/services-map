# services-map

A lightweight microservices registry that serves as a structured data source for AI agents working with microservice architectures. It stores service definitions — including gRPC relationships, message queue bindings, and arbitrary metadata — and exposes them through a simple REST API that AI agents can query to understand, reason about, and operate on your infrastructure.

## Data Model

Each service entry describes a single microservice and its integration points:

```json
{
  "name": "billing-service",
  "type": "backend",
  "github_repo": "org/billing-service",
  "description": "Processes payments and generates invoices",
  "grpc_servers": ["billing.BillingService"],
  "grpc_clients": ["users.UserService", "notifications.NotificationService"],
  "queue": {
    "publish_queues": ["invoices.created", "payments.processed"],
    "subscribe_queues": ["orders.confirmed"]
  },
  "is_http_server": true,
  "metadata": {
    "team": "payments",
    "env": "prod",
    "language": "go"
  }
}
```

| Field          | Type              | Description                                       |
|----------------|-------------------|---------------------------------------------------|
| `name`         | string            | Unique service identifier                         |
| `type`         | string            | Service type (e.g. `backend`, `gateway`, `worker`) |
| `github_repo`  | string (optional) | GitHub repository path                            |
| `description`  | string (optional) | Human-readable description                        |
| `grpc_servers` | string[] (optional) | gRPC services this service exposes              |
| `grpc_clients` | string[] (optional) | gRPC services this service calls                |
| `queue`        | object (optional) | Queue publish/subscribe configuration             |
| `is_http_server` | bool (optional) | Whether the service exposes an HTTP API           |
| `metadata`     | map<string,string> | Arbitrary key-value pairs for filtering          |

## API

### Full Service Map

| Method | Endpoint              | Description                          |
|--------|-----------------------|--------------------------------------|
| `GET`  | `/api/services-map`   | Get all services (with optional metadata filtering) |
| `PUT`  | `/api/services-map`   | Replace the entire service map       |

#### Metadata Filtering

Filter services by metadata using the `meta.` query parameter prefix:

```
GET /api/services-map?meta.team=payments&meta.env=prod
```

Returns only services whose `metadata` contains all specified key-value pairs.

### Single Service CRUD

| Method   | Endpoint                | Description               |
|----------|-------------------------|---------------------------|
| `POST`   | `/api/services`         | Create a new service      |
| `GET`    | `/api/services/{name}`  | Get a service by name     |
| `PUT`    | `/api/services/{name}`  | Update a service by name  |
| `DELETE` | `/api/services/{name}`  | Delete a service by name  |

## Running

### From source

```bash
cargo run
```

The server starts on `http://0.0.0.0:8080`. Data is persisted to a local JSON file.

### Environment Variables

| Variable    | Default              | Description                     |
|-------------|----------------------|---------------------------------|
| `DATA_FILE` | `services-map.json`  | Path to the JSON persistence file |

### Docker

```bash
docker run -p 8080:8080 -v $(pwd)/data:/data -e DATA_FILE=/data/services-map.json ghcr.io/hays-codegang/services-map:latest
```

## Example Usage

Seed the map with your services:

```bash
curl -X PUT http://localhost:8080/api/services-map \
  -H "Content-Type: application/json" \
  -d '{
    "services": [
      {
        "name": "api-gateway",
        "type": "gateway",
        "is_http_server": true,
        "grpc_clients": ["users.UserService", "billing.BillingService"],
        "metadata": { "team": "platform", "env": "prod" }
      },
      {
        "name": "user-service",
        "type": "backend",
        "grpc_servers": ["users.UserService"],
        "queue": { "publish_queues": ["users.created"] },
        "metadata": { "team": "identity", "env": "prod" }
      }
    ]
  }'
```

Query all services owned by a specific team:

```bash
curl http://localhost:8080/api/services-map?meta.team=identity
```

Add a single service:

```bash
curl -X POST http://localhost:8080/api/services \
  -H "Content-Type: application/json" \
  -d '{
    "name": "notification-worker",
    "type": "worker",
    "queue": { "subscribe_queues": ["users.created", "invoices.created"] },
    "metadata": { "team": "platform" }
  }'
```

## AI Agent Integration

This service is designed to be consumed by AI agents that need to understand microservice topologies. Agents can use the API to:

- **Discover dependencies** — query gRPC client/server relationships to trace call chains
- **Understand event flows** — inspect queue publish/subscribe bindings to map async communication
- **Scope changes** — filter by metadata (team, environment, language) to identify blast radius
- **Generate architecture diagrams** — retrieve the full map and produce visual representations
- **Plan migrations** — reason about service coupling and propose refactoring strategies

## Tech Stack

- **Language:** Rust
- **Web framework:** Actix-web 4
- **Persistence:** JSON file on disk
- **Container:** Debian Bookworm slim
- **CI/CD:** GitHub Actions — builds on push/PR, publishes to GHCR on release
