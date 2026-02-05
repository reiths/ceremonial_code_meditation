## Copilot & AI Agent Instructions

**All agent rules and mandatory workflows are defined in [`CLAUDE.md`](../CLAUDE.md).**
Do not duplicate or override those rules here—always reference AGENTS.md for:
- Required agent behaviors
- Output/reporting requirements
- Testing and deployment standards

---

### Project Architecture & Context

- Multi-service, Docker-based system:
  - `services/backend/client-service`: Rust microservice (MongoDB, RabbitMQ)
  - `services/backend/common`: Shared Rust code (e.g., RabbitMQ integration)
  - `db-init/client/init-mongo.js`: MongoDB user/collection setup
  - `rabbitmq/`: RabbitMQ service and healthcheck config
- Orchestrated via top-level `docker-compose.yml` (includes sub-compose files)

### Key Workflows & Integration

- **Start all services:** `docker-compose up -d`
- **Stop services:** `docker-compose down`
- **View logs:** `docker-compose logs -f`
- **Test & coverage:**
  - `cargo test` and `cargo llvm-cov --html` (from `services/backend`)
- **Messaging:**
  - RabbitMQ message types/routing keys: see `common/src/rabbitmq/mod.rs`
  - MongoDB setup: see `db-init/client/init-mongo.js`

### Examples

- To add a message type: update `common/src/rabbitmq/mod.rs` and document the routing key
- To add a service: create a new subdirectory under `services/backend/` and update the relevant `docker-compose.yml`

---
For any unclear or incomplete section, ask for feedback and iterate.