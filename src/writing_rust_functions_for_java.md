# Writing Rust Functions for Java

To expose Rust functions to Java, the functions need to be marked with #[no_mangle] and use the extern "C" calling convention. This prevents Rust from altering the function names and makes them accessible from other languages like Java.

```rust
#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
