# A Rust GraphQL Server with Juniper and Rocket

This repo can be used as a starting point to deploy [Rust](https://www.rust-lang.org) GraphQL servers on Render.
It is based on the official [juniper_rocket](https://github.com/graphql-rust/juniper/tree/master/juniper_rocket) example
and uses [GraphQL Playground](https://github.com/prisma/graphql-playground/) and Juniper `master`.

The sample app is up at https://rust-graphql.onrender.com. Use the query below to start exploring!

```graphql
query {
  human(id: "1002") {
    id
    name
    friends {
      id
      name
    }
    appearsIn
  }
}
```

## Deployment

See the guide at https://render.com/docs/deploy-rust-graphql.
