---
tags: [safety]
level: error
---

Do not use `unwrap()` or `expect()` because it can panic. Use `match` or `if let` to handle errors gracefully

```grit
language rust

and {
    or {
        `$_.unwrap()`,
        `$_.expect($message)`
    },
    within function_item() as $fn where {
        $fn <: not after attribute_item(attribute=attribute(path="test")),
        $fn <: not within mod_item(name="tests")
    }
}
```
