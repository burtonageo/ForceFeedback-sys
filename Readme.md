#ForceFeedback-sys

## Description:

 The `ForceFeedback-sys` library provides bindings to the ForceFeedback library on OSX.
This library provides an API used to manipulate force-feedback compatible devices. Following
the `*-sys` package conventions, the `ForceFeedback-sys` package does not define higher-level
abstractions over the native library.

## Usage:

Add `ForceFeedback-sys` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
ForceFeedback-sys = "*"
```

Then, you can import the `ForceFeedback-sys` in your crate root, and use the functions defined:

```rust
extern crate ForceFeedback_sys as ff;
```

## Contributors:

- [George Burton](https://github.com/burtonageo)

## License
Copyright © 2016 George Burton

Distributed under the [MIT License](License.md).
