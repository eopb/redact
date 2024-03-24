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

The underlying secret contained within the wrapper can only be accessed using the [expose_secret][Secret::expose_secret] method or [expose_secret] function[^1].

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
redact = { version = "0.1", features = ["serde"] }
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

If you would like to implement `Serialize` without exposing the `Secret` see [serde::redact_secret].

## Zeroizing `Secret`s

`redact` does not require `Secret`s to be [`Zeroize`][::zeroize::Zeroize]able but does allow `Secret`s to be `Zeroize`d when the contained secret is `Zeroize`able.
To be able to `Zeroize` `Secret`s, enable `zeroize` in your `Cargo.toml`.

```toml
redact = { version = "0.1", features = ["zeroize"] }
zeroize = "1"
```

Once enabled, it is possible zeroize secrets.

```rust
# use redact::Secret;
use zeroize::Zeroize;


fn main() {
    let mut secret = Secret::new("hunter2".to_owned());

    // [ ... ] use secret here

    // Now that we're done using the secret, zero it out.
    secret.zeroize();
    # assert_ne!(*secret.expose_secret(), "hunter2")
}
```

If you would like your `Secret` to be automatically `Zeroize`d when it is no longer being used,
consider wrapping your `Secret` in [`Zeroizing`][::zeroize::Zeroizing] which will `Zeroize` your secret when it is [`Drop`]ed

```rust
# use redact::Secret;
use zeroize::Zeroizing;


fn main() {
    let mut secret = Zeroizing::new(Secret::new("hunter2".to_owned()));

    // [ ... ] use secret here

    // The secret is automatically zeroed out at the end of the scope when it is dropped
}
```

## Comparison with alternatives

### [secrecy](https://docs.rs/secrecy/latest/secrecy/)

[Secrecy](https://crates.io/crates/secrecy) was the original inspiration for this crate and it has a similar API.

One significant difference is that secrecy requires that all secrets implement [`Zeroize`](https://docs.rs/secrecy/latest/secrecy/trait.Zeroize.html) so that it can cleanly wipe secrets from memory after they are dropped.
This unfortunately limits the types of values that secrecy can wrap in a `Secret` since every type has to be aware of `Zeroize`.

Redact relaxes this requirement, allowing all types to be `Secret`s.
When zeroizing is required, consider the techniques [above](#zeroizing-secrets).

### [secrets](https://docs.rs/secrets/latest/secrets/)

[Secrets](https://crates.io/crates/secrets) provides even stronger memory protection than [secrecy](#secrecy) using [`mlock(2)`]/[`mprotect(2)`] among other things.
If you need strong memory protection before and after a `Secret` is dropped consider [secrets](https://crates.io/crates/secrets).

[`mlock(2)`]: https://man7.org/linux/man-pages/man2/mlock.2.html
[`mprotect(2)`]: https://man7.org/linux/man-pages/man2/mprotect.2.html

[^1]: [Secret] will assume that it is safe to expose its secret to its contained types implemenations of 
[Default],
[Hash],
[Copy],
[Clone],
[Ord],
[PartialOrd],
[Eq],
[PartialEq],
[std::ops::Add],
[std::ops::AddAssign],
[std::ops::BitAnd],
[std::ops::BitAndAssign],
[std::ops::BitOr],
[std::ops::BitOrAssign],
[std::ops::BitXor],
[std::ops::BitXorAssign],
[std::ops::Div],
[std::ops::DivAssign],
[std::ops::Mul],
[std::ops::MulAssign],
[std::ops::Rem],
[std::ops::RemAssign],
[std::ops::Shl],
[std::ops::ShlAssign],
[std::ops::Shr],
[std::ops::ShrAssign],
[std::ops::Sub],
[std::ops::SubAssign],
[std::ops::Neg] and
[std::ops::Not]
