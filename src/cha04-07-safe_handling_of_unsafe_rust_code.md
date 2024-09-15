## Safe Handling of unsafe Rust Code

When exposing `unsafe` Rust code to Java, wrap it in a safe Rust API. This ensures that Java code cannot inadvertently cause undefined behavior.

Example:
```rust
#[no_mangle]
unsafe extern "C" fn dangerous_operation(ptr: *mut i32) {
    if !ptr.is_null() {
        *ptr += 1;
    }
}
```