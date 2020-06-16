# Calculator

Our first programming language is a simple calculator supporting addition and subtraction. This is perhaps the *simplest language* that helps us introducing the major topics from grammar to compilation and virtual machine.

If you haven't cloned the [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust) repo already, please do and navigate to the `calculator` subdirectory.

To start, we have `1 + 1;` in [examples/simple.calc](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/simple.calc) where you can compile with

```text
cargo build --bin main // create a simple executable for Calc
../target/debug/main examples/simple.calc
```

or simply

```text
cargo run --bin main examples/simple.calc
```
