<p align="center">
<a href="https://github.com/kurtbuilds/relativetime/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/relativetime.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/relativetime/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/relativetime.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/relativetime/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/relativetime/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/relativetime">
    <img src="https://img.shields.io/crates/d/relativetime?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/relativetime">
    <img src="https://img.shields.io/crates/v/relativetime?style=flat-square" alt="Crates.io" />
</a>
<a href="https://docs.rs/crates/relativetime">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/relativetime?style=flat-square">
</a>
</p>

# `relativetime`

`relativetime` provides traits on `std::time::Duration` and `chrono::Duration` to easily display human-readable relative times.

```rust
use relativetime::{RelativeTime, NegativeRelativeTime};


fn main() {
    let d = std::time::Duration::from_secs(1);
    assert_eq!(d.relative_time(), "in a few seconds");
    assert_eq!(d.relative_time_in_past(), "a few seconds ago");
    
    let d = chrono::Duration::from_secs(-1);
    assert_eq!(d.relative_time(), "a few seconds ago");
    let d = chrono::Duration::from_secs(1);
    assert_eq!(d.relative_time(), "in a few seconds");
}
```

See the [`docs`](https://docs.rs/relativetime) for the API, and the [tests](https://github.com/kurtbuilds/relativetime/blob/master/src/lib.rs#L88) for more example usage.

# Contributing

Contributions are welcome!