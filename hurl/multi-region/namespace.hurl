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
jsonpath "$.data.namespace.data" count == 2
jsonpath "$.data.namespace.data[0].id" == 1
jsonpath "$.data.namespace.data[0].name" == "canada"
jsonpath "$.data.namespace.data[0].description" == "Flymodel Canada"
jsonpath "$.data.namespace.data[1].id" == 2
jsonpath "$.data.namespace.data[1].name" == "usa"
jsonpath "$.data.namespace.data[1].description" == "Flymodel USA"
