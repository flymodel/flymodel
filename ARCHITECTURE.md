# Architecture

## Objectives

Services exist as islands, where we may opt (in) to specific services (assuming default of all).

The following layers / services:

- Query Layer
  - graphql
- Access Layer
  - authentication
    - oauth2
  - permissioning
  - user storage
- Storage Layer
  - object storage
- Tracking Layer
  - namespaces
  - models
    - artifacts
  - experiments
    - artifacts
- Audit Layer
  - logs
  - events
- Events Layer
  - webhooks

### Non-Objectives

- Non-support via obscure infrastructure dependencies
  - should support common methods of:
    - object storage
    - relational data storage

## Data Model

### Postgres

![rels](./docs/rels.png)
