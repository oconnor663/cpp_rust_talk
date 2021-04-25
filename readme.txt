- references and lifetimes
    - C++ vs Rust table
        - two giant differences: 1) lifetimes always valid, 2) &mut T is never aliased
        - BIG IDEAS SLIDE ^
    - lifetimes
        - simple scopes example
        - string_view example
            - DANGEROUS in C++
        - pushing a string into a Vec example
            - try to return the Vec
    - mutability
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
        - Herb Sutter shared_ptr bug
    - remember that there are basically just big ideas, and the rest follows

- move semantics
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

- case study: File
    - no such thing as a closed file
        - note that there is some debate over whether this was a good idea
    - the drop() function
        - surprise it's empty!
        - but drop() doesn't work on Vec elements or struct fields
    - in general, to close Files in-place, use Option<File>. like in a struct
      field.

- case study: Mutex
    - a container
    - borrowing rules enforce locking
    - Arc<Mutex<T>> is surprisingly convenient

- multithreading and closures
    - BIG IDEA: THE TYPE SYSTEM TRACKS THREAD SAFETY
    - C++ std::for_each(std::execution::par, ...) vs Rust Rayon into_par_iter()
        - trying to increment an int in the loop fails in Rust
    - implicit captures are DANGEROUS in C++

- enums
    - Option
    - Result and ?

- Cargo
