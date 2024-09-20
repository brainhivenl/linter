---
tags: [style]
level: warn
---

Avoid the `class` attribute, use `className` instead

```grit
language js(jsx)

jsx_attribute(name="class", value=$value) => `className=$value`
```
