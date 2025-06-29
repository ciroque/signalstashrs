# signalstashrs

Rust service for collecting and storing various IoT sensor data in a RedisTimeSeries database.

## Features

* Accepts Protobuf-encoded batches of sensor data
* Supports domain tagging of measurements (e.g., SPL, temperature)
* Stores data efficiently in RedisTimeSeries
* Exposes health endpoints (`/healthz`, `/readyz`, `/startupz`)
* Configurable via environment variables

## Usage

### Running Locally

```bash
cargo build --release
./target/release/signalstashrs
```

### Environment Variables

* `BIND_ADDRESS`: IP and port to bind to (default `0.0.0.0:8080`)
* `REDIS_URL`: Redis connection URL (default `redis://localhost:6379`)

### Building Docker Image

```bash
docker build -t signalstashrs .
```

### Kubernetes Deployment

A Helm chart is included under `charts/signalstashrs`.

## Protobuf Schema

See [`proto/sensor.proto`](proto/sensor.proto) for the message definitions.

## License

MIT
