# Arenas
Arenas are a way that Java provides developers to allocate memory in a way
that is particularly useful for creating bindings. Arenas are like a
stack of memory, and its space can be split in various ways, and its lifetime
can be set by various types. The main idea of where arenas can be used is
that they can create space to store objects in Java called Memory
Segments. These memory segments can store data such as
variables, data structures, and functions in a space that the garbage
collector treats differently. That means information stored in
these arenas can be passed to and from foreign functions without worrying about 
whether Java’s garbage collector has tampered with the space.

There are four different types of arenas: confined, automatic, shared, 
and custom. Confined arenas and shared arenas are very similar. They both will live as
long as the Java program unless they are manually closed by the user using
the `.close()` method on the arena object. The key difference between the
two is that confined arenas can only be accessed by a single thread, while
shared arenas can be accessed by multiple threads. This causes a weird
interaction with shared arenas. When a confined arena is closed, its memory
is immediately freed and that’s all there is to it. When a shared arena is
closed, it invalidates all Java references to the space in memory, but it does
not immediately free it as the process takes longer, meaning that the space
in memory is technically alive for a very short amount of time after the
arena is closed. These arenas are useful for creating Rust bindings because they can 
guarantee a space in memory cannot be accessed once closed, so they can be 
implemented into functions to guarantee proper memory safety practices.

The API descriptions for automatic arenas typically vaguely describe their closing 
behavior, such as “the garbage collector eventually frees it automatically”. 
To better describe its behavior, The garbage collector will only free the automatic
arena either at the end of the Java program or when it determines that the
arena is unreachable. But what does the garbage collector see as unreachable?

Testing will show that Java will not close the arena even if every
memory segment inside is set to null. The information inside the arena
has no bearing on the garbage collector’s decision to keep it around. However, a way 
to guarantee that the garbage collector determines the arena as unreachable is to set 
the arena to null. This means that automatic arenas can be useful and reliable for 
creating bindings as well, especially if it is not clear when a certain arena should 
be closed. The only downside of the automatic arena is its interaction with the
garbage collector. It is possible this could cause some sort of increased overhead.

With an Arena, you can call `arena.allocate(size, alignment)` to allocate
memory within the arena. Allocations cannot be individually freed with
Arenas, it’s either all or nothing. Global Arenas
are useful for set-and-forget things, like for loading the Rust library, since this
does not need to be freed. Confined Arenas are good for data that cannot be
safely shared across threads, so for types that don’t implement the Send
trait. Auto Arenas are nice if it is difficult to figure out
when something should be deallocated. Although this isn’t very common as `drop()` 
should be called on Rust objects that require cleanup, and Java’s
garbage collector will not take care of this.

For more information on arenas, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/foreign/Arena.html).
