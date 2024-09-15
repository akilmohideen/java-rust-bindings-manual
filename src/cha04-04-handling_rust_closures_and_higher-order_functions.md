## Handling Rust Closures and Higher-Order Functions

Rust closures are not directly compatible with Java’s FFI. You may need to wrap them in Rust before exposing them to Java or convert them to Java lambdas or `MethodHandles`.

Example:
```rust
#[no_mangle]
extern "C" fn apply_closure(x: i32, f: extern "C" fn(i32) -> i32) -> i32 {
    f(x)
}
```