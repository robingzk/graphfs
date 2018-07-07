# Graphfs
Turn a directory into a GraphQL endpoint. Currently read-only.

Graphfs is written using *Rust*, *Juniper*, and *Rocket*. It is available as
both a binary that can be directly used, and as a library that can be
integrated with an existing server.

## Binary usage
```bash
./graphfs [path]
```

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
