A Firehose of Rust for C++ Programmers

Rust and modern C++ have very similar memory models, and both make heavy use of
references, destructors, RAII, smart pointers, and move semantics. This talk
will be a fast-paced, challenging, and hopefully exciting introduction to how
these familiar C++ concepts work differently in Rust. Side-by-side code samples
will help us gloss over syntax and focus on what's happening in memory,
particularly in cases where the Rust version refuses to compile. We'll see that
memory safety lets Rust embrace "dangerous" features like destructive moves and
borrowing views. We'll also see how types like File and Mutex use ownership and
borrowing to express higher-level invariants and enforce them at compile time.

- things we'll either gloss over or not get to at all
    - syntax
    - slices (i.e. borrowing views)
    - iterators
    - enums (i.e. tagged unions) and Option
    - error handling with Result, ?, and panics
    - traits, polymorphism, static and dynamic dispatch
    - managing dependencies with Cargo
    - unsafe code
    - futures (i.e. coroutines) and async IO

1. references and lifetimes
    - C++ vs Rust table
        - two giant differences: 1) lifetimes always valid, 2) &mut T is never aliased
        - BIG IDEAS SLIDE ^
    a. lifetimes
        - simple scopes example
        - string_view example
            - DANGEROUS in C++
        - pushing a string into a Vec example
            - try to do it inside a separate function
            - note that Vec::push() has no lifetime annotations. the lifetime
              is an implicit property of the element type.
    b. mutability
        - simple example with a couple of aliasing variables
        - getting two &T's out of a Vec is fine, but two &mut T's are annoying
            - many possible workarounds to explore in a longer talk:
                - just use indexes
                - split_at_mut
                - iterators
                - RefCell
                - unsafe code
        - pushing into Vec invalidates a reference
            - before Rust didn't like our function without annotations. But
              here, Rust is fine with the function, and it doesn't like the
              caller.
            - note that Rust doesn't care at all about the *body* of the
              function. we can make the whole thing a no-op, and it doesn't
              change the compiler error.
        - Herb Sutter shared_ptr bug https://youtu.be/xnqTKD8uD64?t=1380
    - remember that there are basically just big ideas, and the rest follows

2. move semantics
    - syntactically an assignment. semantically a memcpy. destructive.
        - simple Vec examples, counting constructions and destructions
        - push a String into a Vec. no new allocation.
        - DANGEROUS in C++ (if C++ has destructive move). Safe in Rust.
            - note that you can't use a moved-from variable again
                - you can in C++, and it's not necessarily unsound
            - note that you can't move something with outstanding references
                - ditto
            - similarly you can't move something *through* a reference
                - again, you can in C++
                - containers support this explicitly, as with Vec::remove()
                - also you can swap() thing through references. or use
                  Option::take(), more on this later.
    - everything is movable. (and moves are bitwise and destructive)
        - BIG IDEA SLIDE ^
        - is that surprising?
        - broadly, no self pointers. There are exceptions to this, but you
          won't see them in your first month.
    - Vec::push() works for everything.
        - Note that there is no Vec constructor. Just Vec::new. This is
          true of everything.
        - Vec::push() never runs your code. If it reallocates, that's always a
          realloc() under the hood. Similarly, the Vec::remove() from above is
          always a memmove.
            - Note that std::vector waives its strong exception guarantee "If
              T's move constructor is not noexcept and T is not CopyInsertable
              into *this"
    - Copy and Clone
    - https://stackoverflow.com/questions/22502606/why-is-stdlock-guard-not-movable
    - https://old.reddit.com/r/rust/comments/k7pssg/do_the_compilers_really_create_more_optimal_code/getph0l/
    - https://www.foonathan.net/2017/09/destructive-move/
    - high level tradeoffs:
        - unique_ptr (and std::variant)
            - C++ unique_ptr must hold null, which is annoying and also a branch in the dtor.
            - std::variant::valueless_by_exception
        - Mutex
            - Rust must heap allocate std::sync::Mutex (though parking lot also exists)
            - C++ can't make lock_guard movable without adding a branch
        - strings
            - Rust can't use GCC's version of the small string optimization: https://youtu.be/kPR8h4-qZdk?t=1637
            - GCC can't use memcpy to resize std::vector<std::string>
fn remove<T>(v: &mut Vec<T>, i: usize) -> T {
    assert!(i < v.len(), "index {} is out of bounds", i);
    unsafe {
        let elem_ptr = v.as_mut_ptr().add(i);
        let ret = ptr::read(elem_ptr);
        ptr::copy(elem_ptr.add(1), elem_ptr, v.len() - i - 1);
        v.set_len(v.len() - 1);
        ret
    }
}

3. case study: File
    - no such thing as a closed file
        - no need to allocate a close flag, and no API questions about what
          operations on a closed file should do
        - on the other hand it suppresses close errors, some debate over
          whether this was a good idea
    - the drop() function
        - surprise it's empty!
        - but drop() doesn't work on Vec elements or struct fields
    - in general, to close Files in-place, use Option<File>. like in a struct
      field.
    - https://isocpp.org/wiki/faq/dtors
    - https://stackoverflow.com/questions/28136080/stdvectorerase-exception-safety

4. case study: Mutex
    - a container
    - borrowing rules enforce locking
    - Arc<Mutex<T>> is surprisingly convenient
    - RwLock and taking the wrong kind
    - Curiously Recurring C++ Bugs at Facebook, unique_lock bug https://youtu.be/lkgszkPnV8g?t=1750
    - different method signatures:
      - https://doc.rust-lang.org/std/thread/fn.spawn.html
      - https://docs.rs/rayon/1.5.0/rayon/iter/trait.ParallelIterator.html#method.for_each
      - Rust doesn't actually know what these do on the inside

5. multithreading and closures
    - BIG IDEA: THE TYPE SYSTEM TRACKS THREAD SAFETY
    - C++ std::for_each(std::execution::par, ...) vs Rust Rayon into_par_iter()
        - trying to increment an int in the loop fails in Rust
        - implicit captures are DANGEROUS in C++.
    - Arc vs Rc
        - non-atomic reference counting is DANGEROUS in C++
        - Tokio multithreading and how non-Send types "infect" a future
