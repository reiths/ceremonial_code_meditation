# Overengineering as a service.

This whole repository is a whole bunch of overengineered nothing, a playground for exploring patterns, best practices,
and gists to be applied somewhere else, and overall just ceremonial code meditation.

## Quick Start

```sh
# Start all services
docker-compose up -d

# Or just specific services
docker-compose up -d client-db rabbitmq

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Full cleanup (removes database volumes)
docker-compose down -v
```

> [!NOTE]  
> All databases are stored in Docker volumes for persistence. Use `docker-compose down -v` for a complete cleanup including all data.

In the `services/backend/<service>` folder, run
```sh
sqlx migrate add
sqlx migrate run
sqlx migrate revert
```