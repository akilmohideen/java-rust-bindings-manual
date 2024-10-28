# Uninitialized Memory
Rust allows developers to work with uninitialized memory. All memory that is
allocated during runtime is uninitialized at first, and it will contain garbage
values. Any novice programmer will know
that working with this memory will cause undefined behavior. Regardless,
Rust provides ways of working with uninitialized memory in safe and unsafe
ways.

## Checked
Rust by default doesn’t allow access to a [memory segment](memory_segment.md) that has not
been initialized yet.
This is great for Java-Rust bindings because it ensures that even if an
attempt is made to access
uninitialized memory from the Java side (which would normally be allowed
and would produce undefined behavior) that is being allocated with Rust
through the [FFM API](https://openjdk.org/jeps/454), it won’t produce undefined
behavior or retrieve garbage values.

## Drop Flags
This is related to the concept of [lifetimes](lifetimes.md). Whenever a variable goes out of
scope, suppose a variable named `x` defined as `let mut x = Box::new(0);`, Rust 
assigns the drop
flag, which then pushes the drop function, `drop(x)`, on the stack.
The concept of [ownership](ownership.md) applies here too, where there can be only one pointer to a
memory segment.
Drop flags are tracked on the stack, and Rust decides when to drop a
value during runtime. This is relevant to creating bindings, because even though
Rust may have dropped a value, the Java variable that points to it when
using the FFM API
 usually would not know that
happened. Having access to a drop flag allows for tracking when such
behavior happens, so they can be invalidated from the Java side too.

## Unchecked
Arrays can not be partially initialized, since null does not exist in Rust, so arrays that are defined
have to be fully initialized, with a value to every
section of memory that is represented by the indexes. This can make
developing code harder, especially when trying to work with dynamically
allocated arrays. To solve this, Rust implements the `MaybeUninit` type.
For example, to define an array that may be uninitialized, we would write:
```rust,ignore
let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
    MaybeUninit::uninit().assume_init() };
}
```
This works because the `MaybeUninit` is the only type that can be partially
initialized, and `.assume_init()` makes the Rust compiler think that the array
of `MaybeUninit<T>` was fully initialized. In this case, we are pointing to a
`Box`, which is a container for the type `u32`. The array can then be initialized
with the following:
```rust,ignore
for i in 0..SIZE {
    x[i] = MaybeUninit::new(Box::new(i as u32));
}
```
Usually, when working with an array of pointers, assigning a new value to
`x[i]` would mean that the left hand side value would be dropped. But this is not
a problem when the left hand side contains `MaybeUninit<Box<u32>>`
because it does not contain anything, it just works as a placeholder. Finally,
that array that may be uninitialized may be turned into an array that we know
has been uninitialized with this line of code:\
`unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }`

To learn more about checked uninitialized memory, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [5.1](https://doc.rust-lang.org/nomicon/checked-uninit.html).
