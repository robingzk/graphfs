# graphfs
Turn a directory into a GraphQL endpoint. Currently read-only.

## Example queries
Get all the entries in the root folder, and show the content of the files:
```graphql
query {
  root {
    name,
    ... on File {
      content
    }
  }
}
```

Read the content of a specific file:
```graphql
query {
  file(path: "/src/main.txt") {
    content
  }
}
```
