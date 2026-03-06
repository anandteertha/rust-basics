# Borrowing and References, Quick Notes (Rust)

## The mental model
- Ownership is “who is responsible for freeing this value”.
- A reference is a temporary permission to access a value owned elsewhere.
- Borrowing is Rust enforcing safe access while the owner still exists.

## Shared reference: `&T`
What it means:
- Read-only access
- Does not transfer ownership
- Many shared borrows can exist at the same time

What Rust guarantees:
- No one can mutate the value while it is shared-borrowed

Typical use:
- Passing data into a function for reading, logging, formatting, comparisons

## Mutable reference: `&mut T`
What it means:
- Read-write access
- Does not transfer ownership
- Must be exclusive (only one mutable borrow at a time)

What Rust guarantees:
- No one else can read or write the value while it is mutably borrowed

Typical use:
- Passing data into a function that updates it in place

## The two golden rules
1) Any number of `&T` OR
2) Exactly one `&mut T`
But never both at the same time.

This prevents data races and invalid reads, even in single-threaded code.

## Borrow scope (the part people miss)
A borrow lasts until the last time the reference is used.

Practical consequence:
- If you create a reference and keep using it later, the borrow is still active.
- If you stop using it, Rust can consider the borrow ended, and allow other borrows.

So most “borrow checker fights” are fixed by:
- narrowing scope, or
- reducing how long you keep the reference alive

## Why this matters (systems view)
These rules are what let Rust avoid a garbage collector while still being safe.
They also set you up for safe concurrency later, because the same exclusivity rules prevent races.

## Quick self-check questions
- Why can I call a function with `&T` and still use the value after?
- Why does `&mut T` block all other access while it exists?
- In my code, where exactly does the borrow begin and where does it end?
- Which line is creating overlap between immutable and mutable access?