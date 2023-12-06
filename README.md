# FlyModel

## Query Layer

```graphql
{
  namespace(name: "canada") {
    data {
      models {
        data {
          id
          name
          lastModified
          createdAt
          versions {
            data {
              id
              version
              state {
                state
              }
              experiments {
                data {
                  id
                  artifacts {
                    data {
                      id
                      name
                      object {
                        id
                        bucketId
                        key
                        sha256
                        encode
                        archive
                        createdAt
                      }
                    }
                  }
                }
              }
              artifacts {
                data {
                  id
                  name
                  extra
                  object {
                    id
                    bucketId
                    key
                    encode
                    archive
                    createdAt
                    sha256
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```
