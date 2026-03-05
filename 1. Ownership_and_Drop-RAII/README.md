# P001: Ownership and Drop (RAII) in Rust

This micro project makes Rust’s cleanup behavior visible.

You create a small `Tracker` type that prints when it is created and when it is dropped, so you can *see*:
- scope based lifetimes
- ownership moves
- deterministic cleanup via `Drop` (RAII)

## Folder layout

- `src/main.rs`
- `src/tracker.rs`

## What you built

`Tracker` is an owned value that holds a heap allocated `String` name. When a `Tracker` goes out of scope, Rust automatically runs its destructor (`Drop`) and you print a line.

### Why this matters

In systems code, you constantly manage resources like files, sockets, locks, and memory buffers. Rust’s core guarantee is:
- cleanup is deterministic (no GC pauses)
- unsafe patterns like double free and use after free are prevented by ownership rules

## How to run

1. Make sure Rust is installed:
   - `rustc --version`
   - `cargo --version`

2. From this project directory:
   - `cargo run`

## Expected output

Your output should follow this pattern:

- outer tracker drops last (end of `main`)
- inner tracker drops when its `{ ... }` block ends
- a tracker moved into a function drops when that function ends
- a tracker passed to `std::mem::drop(...)` drops at that line

Example output from this project:

```
Created tracker for `Outer tracker`
Created tracker for `Inner tracker`
Dropping tracker for `Inner tracker`
Created tracker for `Give away tracker`
tracker Give away tracker is moved to another function
Dropping tracker for `Give away tracker`
Created tracker for `Destroy Tracker`
Dropping tracker for `Destroy Tracker`
Dropping tracker for `Outer tracker`
```

## Workflow for experimenting

Keep changes small and intentional. Try one experiment at a time and observe the print order.

### Experiment A: prove “move” invalidates the old name

After calling `move_tracker_to_me(tracker_to_give_away);`, try using `tracker_to_give_away` again. It should fail to compile because ownership moved.

Goal: internalize “one owner at a time.”

### Experiment B: borrow instead of move (preview of P002)

Change the function to take `&Tracker` (a shared reference) and pass `&tracker_to_give_away`.

Goal: see that you can still use the original variable after the call.

### Experiment C: drop order in reverse creation order

Create three trackers in the same scope: `a`, `b`, `c`.

Goal: observe that they drop in reverse order at scope end.

### Experiment D: explicit early cleanup

Use `std::mem::drop(x)` to release resources earlier than the end of scope.

Goal: understand that this is a function that consumes ownership, not a manual call to the `Drop` trait method.

## Notes and conventions for this rust-learning folder

- Each project is a micro project with a single concept focus.
- Keep the project to 1 or 2 files unless the topic truly needs another small file.
- If a topic feels big, split it into multiple micro projects rather than growing this one.

## Next micro projects in this track

- **P002: Borrowing (shared vs mutable references)**
- **P003: Lifetimes (only the practical slice you need)**
- **P004: Copy vs Move vs Clone, cost awareness**
- **P005: Result and error boundaries**

