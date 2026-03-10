# Project 004 - Connection Tracker

A tiny Rust project to learn how `struct`s and `impl` blocks work together.

This project models a simple network connection and uses methods to mutate and inspect its state. It is intentionally small so the focus stays on the fundamentals:
- grouping data with a `struct`
- attaching behavior with `impl`
- using associated functions like `new()`
- understanding `&self` vs `&mut self`
- modeling state changes cleanly

---

## What this project teaches

This project helps build intuition for one of Rust’s core ideas:

A value can store data and expose behavior, but Rust does this explicitly.

You learn:
- how to define a `struct`
- how to create an instance using an associated function
- how methods operate on an existing value
- why mutation requires `&mut self`
- how to keep fields private and interact through methods
- how to model simple state transitions

---

## Project idea

We build a `Connection` that stores:
- an `id`
- an `address`
- whether it is active
- how many retry attempts have happened

The program:
1. creates a connection
2. tries to increment retries before activation
3. activates the connection
4. increments retries
5. deactivates the connection
6. prints the final state

This makes the project feel more realistic than a plain data container because behavior depends on the current state.

---

## Project structure

```text
Project_004_Connection_Tracker/
├── src/
│   ├── main.rs
│   └── connection.rs
└── Cargo.toml