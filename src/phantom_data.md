# Phantom Data
Sometimes, when working with unsafe code, there may be a situation where
[lifetimes](lifetimes.md) are associated with a struct, but not part of a field. For example:
```rust,ignore
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
}
```
‘a isn’t being used in the body of this struct, so it’s unbounded. In Rust,
making these types of lifetime annotations for structs is not allowed because
of the implications it would have with maintaining correct variance and drop
checking. The solution Rust offers is `PhantomData`, which is a special marker
type. It doesn’t take up more memory, but it simulates a field of the desired struct
type to implement for static analysis. It is easy to implement, the
resulting struct would be:
```rust,ignore
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a T>,
}
```
This way, the lifetime will be bounded to a “field” of the struct `Iter`. This may
bring up complications when writing a tool that automatically generates
bindings to call code because of the way it is designed. As previously
explained, [method handles](method_handle.md) must be written for the different types a
function may be working with, and the [FFM API](https://openjdk.org/jeps/454)
may be incompatible or unable to accommodate for a case where
`PhantomData` is used.
Rust uses [unwinding](unwinding.md) to handle panics (unexpected errors) by default.
In this code, any panic (e.g., an out-of-bounds error) would unwind the stack
safely, cleaning up as it goes. Rust allows opting out of unwinding with
`panic=abort` for faster binaries.

To learn more about phantom data, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [3.10](https://doc.rust-lang.org/nomicon/phantom-data.html).
