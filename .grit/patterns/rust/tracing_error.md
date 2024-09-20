---
tags: [tracing]
level: error
---

Errors should be cast to `&dyn Error` when setting the `error` field of a span

```grit
language rust

`tracing::error!($expr)` where {
    $expr <: contains token_tree($tokens),
    $tokens <: contains `error`,
    not $tokens <: contains `dyn`
}
```

## Error without cast

```rust
tracing::error!({ error = ?e }, "something went wrong");
```

```rust
tracing::error!({ error = ?e }, "something went wrong");
```
