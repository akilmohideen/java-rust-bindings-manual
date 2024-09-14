# Compiling Rust Code to a Shared Library

To use Rust functions in Java, you need to compile the Rust code into a shared library. Add the following to your Cargo.toml:

```toml
[lib]
crate-type = ["cdylib"]
```

Compile the Rust code using:

```bash
cargo build --release
```

This will generate a shared library (e.g., .so on Linux, .dll on Windows, .dylib on macOS) in the target/release/ directory.
