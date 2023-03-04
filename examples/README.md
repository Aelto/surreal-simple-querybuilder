# Example
> It is recommended to read the examples from top to bottom, and to read the comments
> starting with the 👇 emoji.


| File |Description | Crate features |
|------|------------|----------------|
| [`0-querybuilder-basics.rs`](./0-querybuilder-basics.rs) |displays a basic usage of the `QueryBuilder` type | `querybuilder` (default) |
| [`1-model-basics`](./1-model-basics.rs) | demonstrates how the `model` macro is used and where it can help you | `querybuilder`, `model` |
| [`2-model-foreign-nodes`](./2-model-foreign-nodes.rs) | continues on the `model` macro and explains the foreign nodes | `querybuilder`, `model`, `foreign` |
| [`3-model-edges`](./3-model-edges.rs) | continues on the `model` macro too and explains edges | `querybuilder`, `model` |
| [`4-querybuilder-conditional`](./4-querybuilder-conditional.rs) | shows how the `QueryBuilder` type can be used for dynamic queries based on conditions | `querybuilder`, `model` |
| [`5-model-serializer`](./5-model-serializer.rs) | explains what the `pub` keyword does in the `model` macro and how it can be used in the QueryBuilder as well | `querybuilder`, `model` |
| [`6-queries-and-params`](./6-queries-and-params.rs) | demonstrates how to use the premade queries offered by the crate and now they can be enhanced with the custom param types | `model`, `queries` |
| [6-bis](../tests/src/surrealdb_client.rs) | shows a complete example of the premade queries, the params and how to use them with the official surrealdb client. | `querybuilder`, `model`, `foreign`, `queries` (or `all`) |
