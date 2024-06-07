---
tags: [style]
level: error
---

Variable names should use camelCase

```grit
language go

or {
    `$ident := $_`,
    `var $ident = $_`,
    `const $ident = $_`
} where {
    $name = text($ident),
    $name <: r".+_.+"
}
```