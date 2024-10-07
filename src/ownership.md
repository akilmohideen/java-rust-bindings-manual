# Ownership
A piece of data must be owned by at most one variable at any given time,
even across an FFI boundary. If Rust has ownership of a `Vec<T>` for
instance, Java cannot decide to take control of it, as, in this case, that would
lead to both Java and Rust calling drop when done with the type, causing a
double free of the backing array. And that’s one of the better outcomes, as
generally types do not expect to suddenly be in an invalid state due to
external mucking, nor is there much they can do about it. One exception to
this rule are types that implement Copy, as they can be blindly memcopied
to create an identical clone of the original (barring any atomicity issues if
this is done across threads), though most types do not implement Copy so
this isn’t very useful when creating these bindings.

## Example of Ownership
In this calculator code, ownership is demonstrated in how PostfixCalculator
manages its stack:

```rust,ignore
struct PostfixCalculator {
    stack: VecDeque<f64>,
}

impl PostfixCalculator {
    fn new() -> Self {
        PostfixCalculator {
            stack: VecDeque::new(),
        }
    }
}
```

PostfixCalculator owns its stack. When PostfixCalculator is
dropped, so is its stack, which automatically cleans up without the
programmer needing to manually manage memory.

To learn more about ownership, it is recommended to read these official Rust 
resources: *The Rust Programming Language* chapter 
[4](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), and *The Rustonomicon* chapter [6](https://doc.rust-lang.org/nomicon/obrm.html).
