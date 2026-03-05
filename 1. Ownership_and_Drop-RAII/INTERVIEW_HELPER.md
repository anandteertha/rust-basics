# Interview Helper: Ownership and Drop (P001)

## The 45 second explanation (memorize this)

Rust achieves memory safety without a garbage collector using ownership. Each value has exactly one owner. When ownership moves, the old binding is no longer valid, which prevents double frees and use after free. When the owner goes out of scope, Rust runs cleanup deterministically using `Drop`, which is RAII. This model extends beyond memory to resources like file handles, sockets, and locks.

## Core definitions you should be able to say cleanly

### Ownership
A value has a single owner at a time. The owner is responsible for cleanup.

### Move
Assigning or passing an owned value often transfers ownership. After a move, the previous variable cannot be used.

### Borrowing
Instead of moving, you can lend access via references:
- `&T` for shared read only access
- `&mut T` for exclusive mutable access

Borrowing is the next project (P002), but you should already know the intent.

### Drop and RAII
`Drop` is Rust’s destructor hook. Rust calls it automatically at scope end.
RAII means resources are acquired and released with object lifetime, so cleanup is predictable.

### `std::mem::drop(x)`
This is a function that consumes `x` and drops it immediately. You can use it to release a resource earlier than scope end.
You cannot manually call the `Drop` trait method on a value.

## What your P001 demo proves

Your print order demonstrates four interview relevant facts:

1. **Scope based lifetime**
   - inner values drop when their scope ends

2. **Move into a function**
   - passing `Tracker` by value makes the function the owner
   - the value drops when the function returns

3. **Early cleanup**
   - `std::mem::drop(x)` drops at that exact line

4. **Deterministic teardown**
   - outer values drop last, at the end of `main`

## Common interview questions and crisp answers

### Q: How is Rust different from C++ here?
Rust also uses RAII style deterministic destruction, but ownership and borrowing rules are enforced by the compiler, which prevents many memory safety bugs without needing a garbage collector.

### Q: How does Rust prevent double free?
Only one owner exists. You cannot accidentally have two owners of the same allocation. After a move, the old binding is invalid.

### Q: How does Rust prevent use after free?
A reference cannot outlive the value it points to. The compiler checks this. In P001, you never observe a dangling reference because you cannot create one in safe Rust.

### Q: Can you call `drop` manually?
You cannot call the `Drop` trait method directly. But you can call `std::mem::drop(x)` to drop an owned value early.

### Q: Why is deterministic cleanup valuable?
It matters for systems programming because resources like sockets and file handles must be released quickly and predictably. It also helps performance because you avoid GC pauses.

## Whiteboard version (no code)

Draw three boxes for scope:
- `main` scope box
- inner block scope box inside it
- function scope box for `move_tracker_to_me`

Then label a single owner arrow:
- `Tracker` owned by `main`
- `Inner tracker` owned by inner block
- `Give away tracker` ownership arrow moves from `main` to function

Finally, mark cleanup at each scope boundary:
- inner block boundary triggers drop for inner tracker
- function return boundary triggers drop for moved tracker
- end of `main` triggers drop for outer tracker

If you can explain this diagram, you are ready.

## Pitfalls interviewers like

- Thinking moves are copies
- Thinking `drop()` is a method you call like in other languages
- Confusing stack vs heap with “fast vs slow” in a simplistic way
- Forgetting that `String` owns heap memory and the binding itself lives on the stack

## Micro drills (5 minutes)

1. Predict drop order for three variables created in one scope.
2. Predict drop order when you nest scopes.
3. Explain what changes if you pass `&Tracker` instead of `Tracker`.
4. Explain when you would use `std::mem::drop` in real code.

## Next learning chunks that interviewers care about (small, project sized)

### P002: Borrowing rules
Goal: explain why `&mut` is exclusive and how that prevents data races.

### P003: Lifetimes, practical only
Goal: explain “a reference must not outlive its referent” and read compiler errors calmly.

### P009 to P012: Concurrency
Goal: connect ownership and borrowing to thread safety, `Send`, `Sync`, mutexes, and channels.

## Your “tell me about Rust” one liner

Rust is a systems language that gives you deterministic resource management like C++, but uses ownership and borrowing rules enforced at compile time to prevent entire classes of memory and concurrency bugs.
