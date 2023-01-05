# Redact

[![License](https://img.shields.io/crates/l/redact.svg)](https://crates.io/crates/redact)
[![Latest version](https://img.shields.io/crates/v/redact.svg)](https://crates.io/crates/redact)
[![Latest Docs](https://docs.rs/redact/badge.svg)](https://docs.rs/redact/)
[![downloads-badge](https://img.shields.io/crates/d/redact.svg)](https://crates.io/crates/redact)

[API docs](https://docs.rs/redact/)

A simple library for keeping secrets out of logs.

Redact provides a wrapper that prevents secrets from appearing in logs.

```rust
use redact::Secret;

let encryption_key = Secret::new("hello world");
assert_eq!("[REDACTED &str]", format!("{encryption_key:?}"))
```

The underlying secret contained within the wrapper can only be accessed using the [expose_secret][Secret::expose_secret] method or [expose_secret] function.

```rust
use redact::Secret;

let encryption_key = Secret::new("hello world");
assert_eq!("hello world", *encryption_key.expose_secret())
```

The `Secret` type doubles as a useful documentation tool.
Documenting values maintainers should be careful with.

```rust
use redact::Secret;

#[derive(Debug)] // Safe since Debug is not able to "see" our `Secret`s
struct Payment {
    // The recipient is PII so we don't want it to appear in logs
    recipient: Secret<String>,
    // It's okay for the amount to appear in logs so we don't mark it with `Secret`
    amount: u64,
}
```

## Serde support

For serde support ensure the serde feature is enabled in your `Cargo.toml`.

```toml
redact = { version = "0.0.10", features = ["serde"] }
```

`Deserialize` works as expected, transparently deserializing the enclosed secret.

Since serialization can expose the enclosed secret it is only possible to implement `Serialize` "with" [expose_secret].

```rust
use redact::{Secret, expose_secret};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Payment {
    #[serde(serialize_with = "expose_secret")]
    recipient: Secret<String>,
    amount: u64,
}
```

## Comparison with alternatives

### [secrecy](https://docs.rs/secrecy/latest/secrecy/)

[Secrecy](https://crates.io/crates/secrecy) was the original inspiration for this crate and it has a very similar API.

One significant difference is that secrecy requires that all secrets implement [`Zeroize`] so that it can cleanly wipe secrets from memory after they are dropped.
This unfortunately limits the types of values that secrecy can wrap in a `Secret` since every type has to be aware of `Zeroize`.

Redact relaxes this requirement, allowing all types to be `Secret`s. If you need zeroization consider [secrecy](https://crates.io/crates/secrecy).

### [secrets](https://docs.rs/secrets/latest/secrets/)

[Secrets](https://crates.io/crates/secrets) provides even stronger memory protection than [secrecy](#secrecy) using [`mlock(2)`]/[`mprotect(2)`] among other things.
If you need strong memory protection before and after a `Secret` is dropped consider [secrets](https://crates.io/crates/secrets).

[`Zeroize`]: https://docs.rs/secrecy/latest/secrecy/trait.Zeroize.html
[`mlock(2)`]: https://man7.org/linux/man-pages/man2/mlock.2.html
[`mprotect(2)`]: https://man7.org/linux/man-pages/man2/mprotect.2.html

