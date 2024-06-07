---
tags: [style]
level: error
---

Variable names should use underscore_case

```grit
language rust

or {
    `let $ident = $_;`,
    `let $ident: $_ = $_;`
} where {
    $name = text($ident),
    $name <: r".+[A-Z].+"
}
```