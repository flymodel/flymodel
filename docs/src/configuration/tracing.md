# Tracing

## Keys

### `tracing.otlp.target`

The target URL to send spans to.

### `tracing.otlp.tls.ca_source`

The ca chain used to validate requests to a TLS bound exporter. Optional.

### `tracing.otlp.tls.certs.cert_file`

The certificate to send (client) to the otlp exporter. Optional.

### `tracing.otlp.tls.certs.key_file`

The certificate key to send (client) to the otlp exporter. Optional.

## Example

```toml
[tracing]
otlp = { target = "localhost:4317" }
```
