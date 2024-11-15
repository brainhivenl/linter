---
tags: [style]
level: error
---

Variable names should use underscore_case

```grit
language hcl

function cleanup($name) js {
    return $name.text[0].toLowerCase() + $name.text.substr(1).replace(/([A-Z])/, (_, p1) => `_${p1.toLowerCase()}`).replace(/-/g, "_")
}

`variable $ident { $body }` where {
    $ident <: contains r".*[A-Z-].*",
    $ident => cleanup(text($ident))
}
```
