## Managing Multi-threaded Rust Applications

When using multi-threaded Rust code with Java, ensure that data races are avoided and that synchronization is handled properly.

Example:
```rust
use std::sync::Mutex;

struct SharedData {
    counter: Mutex<i32>,
}

#[no_mangle]
extern "C" fn increment(shared_data: &SharedData) {
    let mut counter = shared_data.counter.lock().unwrap();
    *counter += 1;
}
```