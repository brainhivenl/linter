---
tags: [style]
level: error
---

Error messages shouldn't start with a capital

```grit
language go

`$obj.$member($message)` where {
    $obj <: or { `zap`, `logger`, `log` },
    $member <: or { `Info`, `Infof`, `Warn`, `Warnf`, `Error`, `Errorf`, `Log`, `New` },
    $message <: r"\"[A-Z].*"
}
```
