# Calculator

Our first programming language is a simple calculator supporting addition and subtraction. This is perhaps the *simplest language* that helps us introduce the major topics from grammar to compilation and virtual machines.

If you haven't cloned the [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust) repo already, please do and navigate to the `calculator` subdirectory.

To start, we have `1 + 1;` in [examples/simple.calc](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/simple.calc). First, build the project:

```bash
cargo build --bin main
```

This creates an executable at `../target/debug/main`. Now run it on the example file:

```bash
../target/debug/main examples/simple.calc
```

Or simply combine both steps with:

```bash
cargo run --bin main examples/simple.calc
```
