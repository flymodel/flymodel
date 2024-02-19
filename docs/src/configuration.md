# Configuration

## Formats

### Static

Configuration files are searched in `config/flymodel` (with extension added). The following file formats are supported:

- `toml`
- `yaml`
- `json`
- `json5`

### Environment

Environment variables are searched by the following methodology:

1. Replace `.` in a key with `_`.

   `storage.s3` -> `storage_s3`

2. Convert to screaming snake case.

   `storage_s3` -> `STORAGE_S3`

3. Prefix with `FLYMODEL_`.

   `FLYMODEL_STORAGE_S3`

## Options

- [Logs](./configuration/logs.md)
- [Retention](./configuration/retention.md)
- [Storage](./configuration/storage.md)
- [Server](./configuration/server.md)
- [Tracing](./configuration/tracing.md)


