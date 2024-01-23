POST http://localhost:9009/graphql
```graphql
query {
    namespace {
        data {
            id,
            name,
            description
        }
    }
}
```
HTTP 200
[Asserts]
jsonpath "$.data.namespace.data" count == 1
jsonpath "$.data.namespace.data[0].id" == 1
jsonpath "$.data.namespace.data[0].name" == "canada"
jsonpath "$.data.namespace.data[0].description" == "Flymodel Canada"

POST http://localhost:9009/graphql
```graphql
mutation {
  createNamespace(name:"My Namespace", description:"My Description") {
    id,
    name,
    description
  }
}
```
HTTP 200
[Asserts]
jsonpath "$.data.createNamespace.id" == 2
jsonpath "$.data.createNamespace.name" == "My Namespace"
jsonpath "$.data.createNamespace.description" == "My Description"
