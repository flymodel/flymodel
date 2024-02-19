# Artifacts

Artifacts may be stored in two methods:

1. Against a specific model version
2. Against a specific experiment run (of a model version)

It is advised to utilize the file encoding & compression parameters when available, as this will allow for analysis of experiment artifacts between runs.

## Common Metadata Stored

- sha256 sum
- Content-type
- Compression parameters
- File name
- Artifact name

## Model Version Artifacts

Model version artifacts provide an artifact name, and additional optional JSON encoded metadata.

## Experiment Artifacts

Experiment artifacts provide an artifact name.
