# FlyModel

## Query Layer

```graphql
{
  namespace(name: "canada") {
    data {
      models(page: { size: 25, page: 0 }) {
        totalPages
        totalItems
        page {
          size
          page
        }
        data {
          id
          name
          versions {
            totalItems
            data {
              state {
                id
                state
                lastModified
              }
            }
          }
          lastModified
        }
      }
    }
  }
}
```
