# Redact

A simple library for keeping secrets out of logs.

Redact provides a wrapper that prevents secrets from appearing in logs.

```rust
use redact::Secret;
let encryption_key = Secret::new("hello world");
assert_eq!("[REDACTED &str]", format!("{encryption_key:?}"))
```

The underlying secret contained within the wrapper can only be accessed using the [ExposeSecret] trait.

```rust
use redact::Secret;
use redact::ExposeSecret;
let encryption_key = Secret::new("hello world");
assert_eq!("hello world", *encryption_key.expose_secret())
```

The `Secret` type doubles as a useful documentation tool.
Documenting values mantainers should be careful with.

## Comparison with alternatives

### [secrecy](https://docs.rs/secrecy/latest/secrecy/)

[Secrecy](https://crates.io/crates/secrecy) was the original inspiration for this crate and it has a very similar API.

One significant differnece is that secrecy requires that all secrets implement [`Zeroize`] so that it can cleanly wipe secrets from memory after they are dropped.
This unfortiantly limits the types of values that secrecy can wrap in a `Secret` since every type has to be aware of `Zeroize`.

Redact relaxes this requirment allowing all types to be `Secret`s. If you need zeroization consider [secrecy](https://crates.io/crates/secrecy).

### [secrets](https://docs.rs/secrets/latest/secrets/)

[Secrets](https://crates.io/crates/secrets) provides even stronger memory protecton than [secrecy](#secrecy) using [`mlock(2)`]/[`mprotect(2)`] among other things.
If you need strong memory protection before a `Secret` is dropped consider [secrets](https://crates.io/crates/secrets).

[`Zeroize`]: https://docs.rs/secrecy/latest/secrecy/trait.Zeroize.html
[`mlock(2)`]: https://man7.org/linux/man-pages/man2/mlock.2.html
[`mprotect(2)`]: https://man7.org/linux/man-pages/man2/mprotect.2.html
