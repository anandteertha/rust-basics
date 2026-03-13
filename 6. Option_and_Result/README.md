# connSim

**connSim** is a small Rust CLI application that simulates a connection lifecycle in memory.

It was built as a concept-rich Rust project to practice core language features through a simple state-driven system instead of a generic toy example. The project models connection creation, activation, connection attempts, success, failure, inspection, and listing through a command-line interface and an interactive REPL mode.

---

## Why this project

This project was designed to bring together several important Rust concepts in one compact application:

- structs and `impl`
- enums and `match`
- `HashMap`
- mutable borrowing
- `Result<T, E>`
- CLI parsing with `clap`
- modular project structure
- REPL-style interaction
- state transition modeling

Rather than focusing on persistence or external systems, the project stays intentionally in-memory so the learning stays centered on Rust fundamentals and clean logic.

---

## What the project does

Each connection in `connSim` has:

- a unique `id`
- an `address`
- an `active` flag
- a lifecycle `status`

Supported lifecycle states:

- `Disconnected`
- `Connecting`
- `Connected`
- `Failed`

The user can create connections, move them through lifecycle stages, inspect them, and list all currently stored connections during an active session.

---

## Features

- Create a new connection
- Activate a connection
- Start a connection attempt
- Mark a connection as successful
- Mark a connection as failed
- Disconnect a connection
- Inspect a single connection
- List all connections in memory
- Run in one-shot CLI mode or interactive REPL mode

---

## Project structure

```text
src/
├── main.rs
├── cli.rs
├── connection.rs
└── connection_state.rs