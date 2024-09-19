---
tags: [style]
level: warn
---

HTML attributes should use double quotes

```grit
language js(jsx)

jsx_attribute(value=$value) where {
    $value <: within jsx_opening_element(),
    $content = trim($value, `'`),
    ! $content <: $value,
    $value => `"$content"`
}
```
