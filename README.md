# convertable-errors &emsp; [[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/convertable-errors.svg
[crates.io]: https://crates.io/crates/convertable-errors

## Usage

Simply add this to your `Cargo.toml`

```toml
convertable-errors = "0.1.0"
```

and (optionally) to your `lib.rs`:

```rust
#[macro_use]
extern crate convertable_errors;
```

## About

This crate defines a macro for deriving From<T> conversions for variants of Rust enums. For example:

```rust
enum ForeignType;

enum MyEnum {
    Variant(ForeignType)
}

impl From<ForeignType> for MyEnum {
    fn from(v: ForeignType) -> Self {
        Self::Variant(v)
    }
}
```

This is how we would typically define conversions from foreign types to enum variants that contain those foreign types. But [**convertable-errors**](https://github.com/dowlandaiello/convertable-errors) provides a declarative Rust macro to generate these conversions for us. Convertable errors can be used to generate these types of conversions for any type of Rust enum (excluding enums with struct variants, for now), but my main use case is error-like enums.

Using convertable-errors, we can generate `From<T>` definitions for our enum variants like so:

```rust
enum ForeignType;

convertable_error! {
    enum MyEnum {
        (Variant(ForeignType), [(ForeignType, Self::Variant)]
    }
}
```
The syntax for defining a convertable enum with convertable-errors is as follows:
- Each variant of an enum must be wrapped in a tuple: `enum MyError { (Variant(ForeignType)), (Variant1) }`
- The first member of the tuple represents your variant. At the moment, only tuple variants and
unit variants are supported bc I'm a lazy fuck.
- The second member of the tuple (optional) represents the types that can be converted into
that variant: `enum MyError { (Variant(ForeignType), [ ... ]), (Variant1) }`
- The members of the convertable types array are each tuples representing the foreign type that
can be converted into your enum and the closure or variant to apply the foreign value to:
`[(ForeignType, Self::Variant)]`. Internally, this second member can be a closure `|x|
Self::Variant(x)`, a unit variant closure `|_| Self::Variant1`, or simply a variant identifier
where the value of the foreign type will be stored: `Self::Variant`. In practice, you can use
this macro for any enum, but I find it most useful for Error-like enums.

NOTE: This isn't a serious project, I might have made some mistakes, so feel free to open a PR
:) This is just a helpful snippet that I use and felt like sharing.

