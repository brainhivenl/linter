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

block(type="variable", labels=$labels) where {
    $labels <: string_lit(content=$content),
    $content <: contains r".*[A-Z-].*",
    $content => cleanup($content)
}
```
