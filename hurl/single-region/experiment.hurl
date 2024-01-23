POST http://localhost:9009/graphql
```graphql
mutation {
  createModel(namespace: 1, name: "llm.sm") {
    id
    name
    namespaceId
  }
  createModelVersion(model: 1, name: "v1.0.1") {
    id
    modelId
    model {
      id
      name
      namespaceId
    }
  }
  createExperiment(modelVersion: 1, name: "ex_abc_1") {
    id
    name
    versionId
  }
}
```
HTTP 200

[Asserts]
jsonpath "$.data.createModel.id" == 1
jsonpath "$.data.createModel.name" == "llm.sm"
jsonpath "$.data.createModel.namespaceId" == 1

jsonpath "$.data.createModelVersion.id" == 1
jsonpath "$.data.createModelVersion.modelId" == 1
jsonpath "$.data.createModelVersion.model.id" == 1
jsonpath "$.data.createModelVersion.model.name" == "llm.sm"
jsonpath "$.data.createModelVersion.model.namespaceId" == 1

jsonpath "$.data.createExperiment.id" == 1
jsonpath "$.data.createExperiment.name" == "ex_abc_1"
jsonpath "$.data.createExperiment.versionId" == 1



POST http://localhost:9009/upload/experiment-artifact

[MultipartFormData]
file: file,experiment_artifact.json; application/octet-stream;
artifact: file,experiment_artifact_payload.json; application/json;


HTTP 200

[Asserts]
jsonpath "$.blob" == 1
jsonpath "$.id" == 1
jsonpath "$.name" == "ABC Test.json"
jsonpath "$.experiment_id" == 1



GET http://localhost:9009/download/experiment-artifact?artifact_id=1

HTTP 200

[Asserts]
header "Content-Type" contains "application/json"
header "Content-Disposition" contains "attachment; filename=\"ABC Test.json\""
header "Digest" contains "sha256=89de6a4d19e7ff70bfa94b5e10ca6994d8e587e95d374928e736e3142c90bc92"

sha256 == hex,89de6a4d19e7ff70bfa94b5e10ca6994d8e587e95d374928e736e3142c90bc92;

jsonpath "$.some.artifact" == "value"

