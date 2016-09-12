# Reverse Polish Notation written in Rust

This package contains both an executable and a library for [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) (aka RPN) calculation.

## Executable

In order to run a REPL to evaluate RPN expressions, just run `cargo run`.

## Library

If you want to evaluate use the library, take a look at `rpn::evaluate(expr: &str) -> Result<f32, &str>`:

```rust
extern crate rpn;

let result:f32 = rpn::evaluate("5 2 +").unwrap();
```

## License

MIT
