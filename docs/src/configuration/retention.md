# Retention

Configuration keys follow per-lifecycle groupings. E.g. `retention.prod.models`.

## `retention.<lifecycle>.models`

Days to retain models in the specified lifecycle. None is forever.

## `retention.<lifecycle>.runs`

Days to retain run details in the specified lifecycle. None is forever.

### Sample

```toml
[[retention.prod]]
models = 365
runs = 28
```
