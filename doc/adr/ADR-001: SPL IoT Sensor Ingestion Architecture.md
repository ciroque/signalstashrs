# ADR-001: SPL IoT Sensor Ingestion Architecture

## Status

Accepted

## Context

We are building a low-power, single-device IoT solution to measure environmental sound pressure levels (SPL) over time. The goal is to quantify noise levels and evaluate changes (e.g., from growing a hedge barrier). The device will operate continuously, sampling at 250ms intervals, and uploading batched data to a backend ingestion service for storage and visualization.

## Decisions

### Hardware

* **Microcontroller**: Arduino Nano 33 IoT

  * Chosen for built-in Wi-Fi (WiFiNINA), low power consumption, small size
* **Sensor**: MAX4466 analog microphone amplifier

  * Provides sufficient accuracy for SPL approximation

### Data Sampling

* **Sampling Interval**: 250 milliseconds (4 Hz)

  * Balances resolution with storage/transmission efficiency
* **Data Format**:

  * `timestamp`: `uint64` (epoch millis)
  * `datum`: `float32`
  * `domain`: enum to describe the measurement type (e.g., SPL)
  * `device_id`: 5-byte fixed identifier (binary)

### Transmission Strategy

* **Batch Interval**: 10–60 seconds (configurable)
* **Batch Format**: Protobuf message:

```protobuf
  syntax = "proto3";
  package sensor;

  enum Domain {
    UNSPECIFIED = 0;
    SOUND_PRESSURE_LEVEL = 1;
  }

  message SensorData {
    uint64 timestamp = 1;
    float datum = 2;
    Domain domain = 3;
    bytes device_id = 4;
  }

  message SensorDataBatch {
    repeated SensorData samples = 1;
  }
```
* **Protocol**: HTTP POST to ingestion endpoint
* **Retry Logic**: Ring buffer in SRAM for unsent samples (resilient to connectivity loss)

### Backend

* **Project Name**: `signalstashrs`
* **Ingestion Language**: Rust

  * `prost` for Protobuf
  * `axum` for HTTP server
  * `redis` crate for TS.ADD commands to RedisTimeSeries
* **Deployment Platform**: Kubernetes

  * Helm chart under `charts/signalstashrs`
  * ENV-driven config via ConfigMap
  * Health endpoints for readiness/liveness
  * Optional Ingress (TLS via CertManager)
* **Storage**: Redis 7.x with RedisTimeSeries module

  * Each device stores values under a `spl:<device_id>` or similar key based on domain
* **Visualization**: Grafana (RedisTimeSeries plugin)

### Security

* **Encryption**: None for now (data not sensitive)
* **Authentication**: Token-based auth to ingestion endpoint (optional)
* **Transport**: HTTP (option to upgrade to HTTPS later via Ingress)

## Consequences

* Simplified architecture, no MQTT/gRPC complexity
* Easy to debug, easy to scale (within reasonable limits)
* Manual flashing + provisioning; no OTA for now
* Resilient to short-term network partitions due to buffer + batch
* Cloud-native deployment pattern enabled via Helm + Kubernetes

## Future Considerations

* Outdoor enclosures + solar power
* OTA firmware updates
* Extended metadata logging (battery, signal strength)
* TLS + auth
* Message queue integration if downstream consumers require event streams
* Helm chart enhancements (autoscaling, Prometheus metrics, ingress annotations)
