# Storage

## S3

S3-compatible object storage.

### Setup

The below command is required to pre-initialize the storage database with the known storage hosts:

`flymodel setup-storage`

### Keys

#### `s3.bucket`

The bucket name.

#### `s3.endpoint`

The target endpoint to connect to.

#### `s3.region`

The region to connect to. Optional if `s3.endpoint` is provided and region is not required.

#### `s3.role`

The role (assigned lifecycle) of the bucket to store artifacts. Required.

#### `s3.public`

Whether assets should be public. Defaults to false.

#### `s3.path_style`

Force use path style object access in the bucket. Default true.

### Sample

```toml
[[storage.s3]]
bucket = "ml-prod"
endpoint = "https://my-minio-endpoint:9000"
region = "ca"
role = "prod"
```
