# Borrowing and Aliasing
Data can be “borrowed” as references, either immutably `&T` or mutably
`&mut T`. The compiler enforces a sort of reader-writer lock on the type: it
can have either multiple readers (immutable/shared references, `&T`) or a
singular writer (mutable/exclusive reference, `&mut T`). The compiler will
assume that the data behind a shared reference will not mutate (unless the
type opts out of it with `UnsafeCell`, which can be used for custom special
types, which should not be used to enforce users’ types) and the compiler
will assume that no other code or data can reference, read, or mutate the
data behind an exclusive reference (there is no opt out, this must never
happen!). The fact that Rust can make these assumptions is what makes it
so fast and efficient, but it also means you are restricted from coding practices 
that break them.

This is approximately the exact opposite of Java’s memory model, where
everything is a mutable reference to the underlying object. While Java can’t
arbitrarily clone objects, meaning it can’t make copies of a class holding an
exclusive reference, it can make those objects live arbitrarily long. This
means it is essential to either detect that the reference is still live and refuse to
service any other borrows, or invalidate the reference in order to service
other borrows. There is a Rust type that effectively performs this latter
approach: `RefCell<T>`.

Raw pointers in Rust do not have such aliasing restrictions with regard to
each other, so we are free to have any number of constant `*const T` and
mutable `*mut T` pointers coexisting. Raw pointer semantics are just like
they are in C, and are in fact even more lenient than C pointers since C
pointers of differing types are not allowed to alias. You’re still not allowed to
mess with ownership – the owner of the type still acts like your pointers
don’t exist and so still assumes it is the arbiter of reads and writes – but if
you have ownership of the type you can just make sure to only interact with
it using raw pointers. This is exactly what `UnsafeCell<T>` and `Cell<T>` do to
enable shared mutability, and those are the primitives fancy types like
`Rc<T>` use to allow shared ownership.

## Example of Borrowing and Aliasing
In this calculator code, Borrowing and Aliasing is demonstrated.

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

Rust's borrowing rules ensure that references to
data (borrowing) do not outlive the data they reference (ownership).
This prevents dangling pointers

To learn more about borrowing and aliasing, it is recommended to read these official Rust 
resources: *The Rust Programming Language* chapter 
[4.2](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html), and *The Rustonomicon* chapters [3.1 and 3.2](https://doc.rust-lang.org/nomicon/).
