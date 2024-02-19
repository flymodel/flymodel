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
