---
tags: [safety]
level: error
---

Do not use `unwrap()` or `expect()` because it can panic. Use `match` or `if let` to handle errors gracefully.

```grit
language rust

and {
    or {
        `$_.unwrap()`,
        `$_.expect($message)`
    },
    within function_item(),
    not within mod_item(name="tests")
}
```
