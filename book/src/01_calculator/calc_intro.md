# Calculator

Our first programming language is a simple calculator supporting addition and subtraction. This is perhaps the simplest language helping us introducing the major topics from grammar to compilation and virtual machine.

If you haven't cloned the [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust) repo already, please do and navigate to the `calculator` crate

```text
git clone https://github.com/ehsanmok/create-your-own-lang-with-rust
cd create-your-own-lang-with-rust/calculator
```

To start, we have `1 + 1;` in [examples/simple.calc](../../../calculator/src/examples/simple.calc) where you can compile with

```text
cargo build --bin main // create the CLI executable for Calc
../target/debug/main examples/simple.calc
```

or simply

```text
cargo run --bin main examples/simple.calc
```
