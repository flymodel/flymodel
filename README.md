# FlyModel

## Overview

Flymodel is a Machine Learning model Version Control & Content Management System with an emphasis on immutability and state validation. It is built in rust, and offers type-validated client libraries in Python and Node.

## Features

- Artifact storage and retrieval via s3-compatible services
- Metadata storage via standard SQL
  - [x] Postgresql
  - [ ] Sqlite / Memory
  - [ ] Mariadb
- Graphql query layer

## Development

### Pre-Requisites

1. [Cargo / Rust](https://rustup.rs)
2. [Docker](https://docker.com)
3. [Hurl](https://hurl.dev)
4. [Tasks](https://taskfile.dev/installation/) (opt)

### Dev Session

#### One-time / on changes

Regarding test data, there are two seed envionment cases which must be satisfied:

- single region / `basic`
- multi region / `multi_region`

Supplement the below $TEST_DATA with the desired test case

<br/>

> [!NOTE]
> You can run `task reset` and skip the below steps

<br/>

1. Start background services

```sh
docker-compose up
```

2. Setup database with test data

```sh
cargo migrate-up --test-data $TEST_DATA
```

#### Serve

```sh
cargo serve
```

There will now be a local api & graphql service running on http://localhost:9009.
