# Send and Sync
* Send: the type can be moved between threads.
* Sync: a type can be shared between threads (logically equivalent to `&T`
being Send)

By default, most types are Send and Sync. If a type is moved to another
thread, it is fine because it owns its data and therefore nothing else can
touch that data or cause thread safety issues. If a shared reference is moved to
another thread, that is fine because the mere existence of a
shared reference means the data can no longer mutate, so there’s nothing
needing synchronization between threads. If an exclusive reference is moved, again 
it is fine because that exclusive reference is the only thing
allowed to look at or modify the underlying data, so there is no need to
synchronize anything. The only types that are not both Send and Sync are
types that cheat the aliasing and ownership rules like `UnsafeCell<T>` and
`Rc<T>`.

Luckily, Java actually allows for this to be enforced. `Arena.ofConfined()` gives us
a thread-local memory [arena](arenas.md), and if code tries to use a `MemorySegment`
allocated from this arena in another thread it will throw an exception. This is
an absolute life saver, as it allows for the use of `RefCell<T>`, which is neither
Send nor Sync, and which is useful for fixing many of the incongruities
between Java and Rust’s memory models.

## Example of Thread Safety and Send and Sync
```rust,ignore
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let calculator = Arc::new(Mutex::new(PostfixCalculator::new()));
    let calculator_clone = Arc::clone(&calculator);
    let handle = thread::spawn(move || {
        let mut calc = calculator_clone.lock().unwrap();
        let tokens: Vec<&str> = "3 4 +".split_whitespace().collect();
        calc.evaluate(tokens)
    });
    
    match handle.join().unwrap() {
        Ok(result) => println!("Result from thread: {}", result),
        Err(e) => println!("Error from thread: {}", e),
    }
}
```
Thread Safety: The Arc and Mutex wrapping of `PostfixCalculator` ensures that
it can be safely shared and mutated across threads. Arc allows for shared
ownership across threads, while Mutex provides mutual exclusion,
preventing [data races](data_races.md).

To learn more about Send and Sync traits, it is recommended to read these official Rust 
resources: *The Rust Programming Language* chapter 
[16.4](https://rust-book.cs.brown.edu/ch16-04-extensible-concurrency-sync-and-send.html), and *The Rustonomicon* chapter [8.2](https://doc.rust-lang.org/nomicon/send-and-sync.html).
