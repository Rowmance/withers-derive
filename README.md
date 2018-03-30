# Withers Derive

A macro to implement wither methods for properties of a struct.

## The Wither Pattern

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

## Contributing