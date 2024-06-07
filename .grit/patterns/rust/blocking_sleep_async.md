---
tags: [behaviour]
level: error
---

Use tokio sleep function so that it doesn't block the scheduler

```grit
language rust

`async fn $name() { $body }` where {
    $body <: contains or {
        `std::thread::sleep($time);`,
        `thread::sleep($time);`
    } => `tokio::time::sleep($time);`
}
```