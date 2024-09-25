---
tags: [style]
level: error
---

Declaring a variable to another variable is useless, please remove it

```grit
language rust

let_declaration(pattern=$pat, value=$val) => `` where {
    $pat <: identifier(),
    $val <: identifier()
}
```

## Useless declaration should be removed

```rust
let y = 1;
let x = y;
```

```rust
let y = 1;
```
