# Withers Derive
[![Latest Version](https://img.shields.io/crates/v/withers_derive.svg)](https://crates.io/crates/withers_derive)

A macro to implement wither methods for properties of a struct.

## The Wither Pattern
The wither pattern consists of using methods to mutate an instance of a struct, generally beginning with `with_`.

```rust
#[derive(Withers, Default)]
struct Foo {
    bar: u32,
    baz: Option<bool>
}

fn main() {
    let instance = foo::default()
        .with_bar(32)
        .with_baz(Some(false));
}
```

This implementation generates consuming methods, so the instance is mutated and consumed, and can no longer be used in its previous state.

### Wither vs Builder
The wither pattern bares strong resemblance to the builder pattern, though:

- The wither pattern does not require a separate struct.
- The wither pattern allows easy mutation of existing instances (this is useful in conjunction with `clone` and/or `copy`).
- The wither pattern will not lead to runtime errors due to uninitialised properties.

_However:_
- The wither pattern requires a struct to be able to be initialised with default and sensible values. Otherwise an error which could be caught on initialisation could cause unexpected behaviour down the line.
- A builder can usually be stored and used to generate several instances of structs, whereas withers operate on instances directly.

If these feel like they may be issues for you, the [derive_builder](https://docs.rs/derive_builder) crate may suit your needs better.

## What it does
The following code
 ```rust
#[macro_use]
extern crate withers_derive;

#[derive(Withers)]
struct Foo {
    bar: u32,
    baz: Option<bool>
}
```
Will generate code akin to
```rust
struct Foo {
    bar: u32,
    baz: Option<bool>
}

#[allow(dead_code)]
impl Foo {
    fn with_bar(mut self, value: u32) -> Self {
        self.bar = value;
        self
    }

    fn with_baz(mut self, value: Option<bool>) -> Self {
        self.baz = value;
        self
    }
}
```

## Quick Start

## License
[MIT License](LICENSE)

## Contributing
Please fork the repository and raise pull requests into master. Larger code changes should be tracked with GitHub issues.