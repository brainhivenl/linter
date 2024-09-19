---
tags: [style]
level: warn
---

HTML attributes should use double quotes

```grit
language js(jsx)

jsx_opening_element(attribute=$attribute) where {
    $attribute <: jsx_attribute(value=$value),
    $content = trim($value, `'`),
    ! $content <: $value,
    $value => `"$content"`
}
```
