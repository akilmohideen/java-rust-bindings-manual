# Subtyping and Variance
As a warning, this section will be complex and type-theory heavy, but the gist for 
this scope
is that there
are three types of lifetime relationships:
* Covariant: ‘a can be used where a ‘b is expected if ‘a is as long or
longer than ‘b. Shared references are covariant because a longer-living reference than required can always be given. Tree structures
where you can only delete leaves kind of act like this (so a
`RefCell<&T>` chain of references follows this).

* Contravariant: ‘a can be used where a ‘b is expected if ‘a lives as long
or shorter than ‘b. This only applies to arguments inside of functions or
closures, so those should be banned from use to avoid any headaches. Closures
aren’t application binary interface safe so they are already banned,
and functions as arguments can be replaced with Java upcalls where
less care is needed.

* Invariant: ‘a cannot be used, the thing you pass in must live exactly as
long as ‘b. This applies to exclusive references, because Rust allows
you to modify data behind an exclusive reference and potentially
change its lifetime, and the caller would have no idea its lifetime got changed, so
that would fail once the caller tries to use it within
its old lifetime, but outside its new lifetime. If an exclusive reference is
checked for validity first before every time it is used, this can work (it’s
effectively `RefCell<&mut T>`), but that then still bans every function
that touches an exclusive reference directly. Honestly, this may not be truly
solvable, it might just have to be invasive to the
programmer.

To learn more about subtyping and variance, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [3.8](https://doc.rust-lang.org/nomicon/subtyping.html).
