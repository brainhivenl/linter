---
tags: [style]
level: warn
---

Inline styling is not allowed in React components, use tailwind instead

```grit
language js(jsx)

`style={{$style}}` where {
    $style <: within jsx_opening_element(name=$name),
    $name <: r"[a-z-]+",
    ! $style <: contains template_substitution()
}
```
