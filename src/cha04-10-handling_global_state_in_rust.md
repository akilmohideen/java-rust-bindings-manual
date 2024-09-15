## Handling Global State in Rust

Global state in Rust must be managed carefully when exposed to Java, particularly in multi-threaded environments.

Example:
```rust
static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
extern "C" fn increment_global() {
    GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
}
```