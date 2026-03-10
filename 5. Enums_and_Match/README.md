# Project 005 - Enums and Match: Connection State Manager in Rust

This project models a small connection lifecycle system in Rust using `enum` and `match`.

The goal is to represent connection state explicitly and make transitions easy to understand through a clean, type-safe design.

Instead of storing status as a string, the program uses an enum to describe the current state of a connection:

- `Disconnected`
- `Connecting`
- `Connected`
- `Failed`

This project also introduces an important design idea:

- some actions are state-aware and controlled
- some actions act like force operations and can happen at any time

In this implementation:

- `start_connecting()` is guarded by connection activity and current state
- `mark_connected()` is guarded by connection activity and current state
- `fail()` can happen at any time
- `reset()` can happen at any time

That makes the project feel closer to a real system.

## Why this project matters

This project demonstrates how Rust enums can model system behavior more clearly than flags or string-based status values.

It also shows how `match` helps keep branching logic explicit and readable.

Key ideas demonstrated:

- enum-based state modeling
- pattern matching with `match`
- controlled state transitions
- method-based API design
- realistic lifecycle simulation

## Project structure

```text
project_005/
├── src/
│   ├── main.rs
│   ├── connection.rs
│   └── connection_state.rs