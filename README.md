# Graphfs
> Turn a directory into a GraphQL endpoint.

Graphfs is written in **Rust** using **Rocket** as a web server and **Juniper**
as a GraphQL library. It is available as a binary that can be directly used,
and as a Rust library.

### Binary usage
```bash
./graphfs [path]
```

### Example queries
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