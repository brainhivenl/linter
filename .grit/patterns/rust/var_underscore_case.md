---
tags: [style]
level: error
---

Variable names should use underscore_case

```grit
language rust

function cleanup($name) js {
    return $name.text[0].toLowerCase() + $name.text.substr(1).replace(/([A-Z])/, (_, p1) => `_${p1.toLowerCase()}`)
}

let_declaration(pattern=$pat) where {
    $pat <: identifier(),
    $pat <: contains r".*[A-Z-].*",
    $pat => cleanup(text($pat))
}
```
