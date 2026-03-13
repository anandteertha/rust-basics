# connSim - Interview Helper

## Project in one line

I built a Rust CLI application called **connSim** that simulates a connection lifecycle using `clap`, `HashMap`, enums, and `Result` to model state transitions in an interactive in-memory session.

---

## 30-second explanation

connSim is a Rust command-line simulator for managing connection objects through lifecycle events like create, activate, connect, succeed, fail, disconnect, inspect, and list. Each connection is stored in an in-memory `HashMap` and modeled using a struct plus a `ConnectionState` enum. I used `clap` for CLI parsing, `Result` for valid and invalid operations, and a REPL loop so multiple commands can operate on the same state during one running session.

---

## Why I built it

I wanted a Rust project that stayed small in code size but still covered a lot of important concepts. A connection lifecycle simulator was a good fit because it naturally involves explicit states, transitions, validation, mutable data updates, and command-driven interaction.

---

## Core concepts demonstrated

- structs and `impl`
- enums
- pattern matching
- `HashMap`
- mutable borrowing
- `Result<T, E>`
- `clap`
- REPL design
- modular Rust project organization

---

## Architecture summary

The project is divided into four modules:

- `main.rs` handles the app flow, REPL loop, command dispatch, and shared connection map
- `cli.rs` defines the command interface using `clap`
- `connection.rs` contains the `Connection` struct and lifecycle methods
- `connection_state.rs` defines the connection state enum

This separation keeps parsing, domain behavior, and state representation cleanly separated.

---

## Best technical talking points

### 1. Explicit state modeling
I used an enum to represent connection state so the lifecycle stays constrained and easy to reason about.

### 2. Transition-driven methods
Operations like `connect` and `succeed` depend on the current state, so methods use `match` and return `Result` to model valid and invalid transitions explicitly.

### 3. In-memory session design
Initially, I stored connections in a `HashMap` inside `main()` and expected state to exist across separate CLI runs. I realized the real issue was process lifetime, not ownership, so I introduced a REPL mode to keep the same in-memory state alive during one session.

### 4. Reusable command execution flow
I extracted repeated lookup and validation logic into a shared helper so each command handler stays smaller and more consistent.

---

## Biggest challenge and what I learned

### Challenge
At first, I thought Rust was somehow “clearing” my `HashMap` between commands.

### What was actually happening
The map was not being cleared by Rust. The process was exiting after each one-shot CLI run, so a new empty map was being created every time the program started.

### Fix
I added a REPL mode so the application can stay alive and keep the same `HashMap` in memory across multiple commands.

### Learning
That taught me an important distinction between:
- ownership and borrowing issues
- process lifetime and state persistence

---

## Good answers to common interview questions

### What does this project do?
It simulates connection lifecycle transitions through a Rust CLI, allowing users to create, activate, connect, succeed, fail, disconnect, inspect, and list connections during an interactive session.

### Why did you use an enum?
Because a connection should only be in one lifecycle state at a time, and enums model that very clearly and safely.

### Why use `Result` here?
Because not every action is valid in every state. Returning `Result` makes success and failure explicit and keeps the logic cleaner than printing everything directly inside the business layer.

### Why use a `HashMap`?
Because connections are identified by id, so key-based lookup is a natural fit and makes the REPL session efficient and simple.

### Why use a REPL instead of only one-shot commands?
Because one-shot commands restart the program each time, which destroys in-memory state. The REPL keeps the application alive so multiple commands can operate on the same data.

### What would you improve next?
I would add a custom error enum, stronger transition validation, dedicated tests, and cleaner formatting for list and inspect output.

---

## Good “what I learned” answer

This project helped me understand how Rust becomes much more expressive when state is modeled explicitly. I also got more comfortable with mutable access through `HashMap::get_mut`, using `Result` for domain behavior, and designing around process lifetime instead of assuming in-memory state persists automatically.

---

## Good “technical depth” answer

Even though the project is small, it involves several real engineering decisions: how to model lifecycle state, how to separate CLI concerns from business logic, how to represent invalid transitions, and how to keep a session alive when state is intentionally in memory only.

---

## Good resume-style description

Built a Rust CLI connection simulator using `clap`, `HashMap`, enums, and `Result` to model lifecycle transitions, support interactive session-based command execution, and enforce explicit state-driven behavior.

---

## Good closing summary for interviews

connSim is a small project, but it shows that I can structure a Rust CLI cleanly, model state explicitly, reason about lifecycle transitions, and make deliberate design choices around in-memory state and command-driven workflows.