## Learn Rust Basics With Me

[![Language: Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Track: Learning in public](https://img.shields.io/badge/track-learning_in_public-blue)](.)
[![Status: Active](https://img.shields.io/badge/status-active-success)](.)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](.)

This repo is a small, focused learning track where **you and I learn Rust systems programming together**.  
Each folder is a micro–project that makes one core idea *visible* through tiny, realistic examples.  
By the end, you will have enough hands‑on practice to **talk confidently about Rust, memory safety, and systems‑design style trade‑offs in interviews**.

## Who this is for

- **You know another language** (C/C++, Java, Go, Python, etc.) and want to pick up Rust quickly.
- **You have upcoming Rust or systems‑programming interviews** and want something concrete to point to.
- **You want proof of real understanding**, not just “I watched a tutorial.”

You can literally show this repo to interviewers as **evidence that you understand ownership, borrowing, lifetimes, safe resource management, type-driven design, and reusable abstraction in Rust**.

## What’s inside (so far)

Each numbered folder is a self‑contained Cargo project with its own `README.md` explaining the concept and exercises.

- **1. Ownership_and_Drop-RAII**  
  Learn how Rust guarantees deterministic cleanup without a GC using ownership and the `Drop` trait.  
  This is the foundation for talking about **resource management, RAII, and avoiding use‑after‑free / double‑free bugs**.

- **2. Borrowing_and_References**  
  Practice shared and mutable references, borrow scopes, and the rules that keep aliasing safe.  
  Great for explaining **how Rust enforces correctness at compile time instead of relying on runtime checks**.

- **3. Slicing-Borrowing parts of data**  
  Work with string and array slices, borrowed views into existing data.  
  This ties directly to **zero‑copy APIs, avoiding unnecessary allocations, and cache‑friendly design**.

- **4. Structs_and_impls**  
  Model a simple `Connection` type with methods and state transitions.  
  This is how you talk about **modeling real systems in Rust using types and invariants instead of ad‑hoc flags**.

- **5. Enums_and_Match**  
  Model a connection lifecycle with an enum (`Disconnected`, `Connecting`, `Connected`, `Failed`) and `match`.  
  Covers **state-aware vs force operations**, controlled transitions, and type-safe state modeling. Great for explaining **explicit state machines and pattern matching** in Rust.

- **6. Option_and_Result**  
  Build toward a small in-memory connection simulator while practicing `Option<T>` and `Result<T, E>`.  
  This gives you language to talk about **error handling, absence vs failure, and how Rust models recoverable vs unrecoverable conditions** in real programs.

- **7. Traits_and_Generics**  
  Build a small connection-monitoring style project with multiple concrete connection types like database, API, and cache.  
  This project teaches **how traits define shared behavior, how generics enable reusable logic across multiple types, and how Rust encourages design around capability instead of hardcoded type branching**.

- **8. Lifetimes_and_Borrowed_Data**  
  Parse system status logs into structs that borrow from the original input, so you can **see how lifetimes connect structs that hold references to the data they point into, and how Rust prevents dangling references when returning borrowed views from parsers**.

- **9. Iterators_and_Closures**  
  Walk through a small alert-style dataset (`Alert` + `AlertLevel`) using **`iter`, `filter`, `map`, `find`, `any`, `all`, and `fold`**, with closures passed into iterator methods.  
  This is where the track shifts toward **idiomatic, declarative data processing** in Rust while ownership and borrowing still apply to what the iterators yield.

The list above will **grow over time** as new micro-projects are added (more ownership patterns, error handling, collections, file handling, async, concurrency, etc.).  
Treat this repo as a **living Rust notebook** that keeps expanding as we learn more.

More micro‑projects can be added over time, always with the same philosophy: **one concept, a tiny program, strong intuition**.

## How to use this repo

1. **Clone the repo**

   ```bash
   git clone <this-repo-url>
   cd rust-basics
   ```

2. **Pick a micro‑project**

   ```bash
   cd "1. Ownership_and_Drop-RAII"
   cargo run
   ```

3. **Read the project README** in that folder, then:
   - Run the code.
   - Make one small change at a time.
   - Re‑run and observe what changed.
   - Trigger at least one compiler error on purpose and understand *why* it happens.

4. **Repeat for the other folders**, in order.

If you are new to Rust, doing **1 → 9 in order** will give you a solid mental model for how Rust’s safety guarantees and abstraction mechanisms work.

## Using this as interview prep

When preparing for Rust or systems‑programming interviews, this repo helps you:

- **Explain memory safety clearly**  
  - Ownership + borrowing + lifetimes as a coherent story.  
  - Why Rust prevents data races, use‑after‑free, and iterator invalidation at compile time.

- **Discuss systems design in Rust terms**  
  - How to model resources (files, sockets, connections) using RAII and `Drop`.  
  - How to design APIs that take `&T`, `&mut T`, or owned `T`, and what that signals about performance and safety.

- **Show type-driven design and reusable abstractions**  
  - How enums model explicit state.  
  - How traits define behavior contracts.  
  - How generics let one function work across multiple concrete types safely.

- **Talk about iterators and closures**  
  - How iterator adapters (`filter`, `map`, `find`, `fold`, etc.) compose, when work actually runs (lazy vs consumed), and how closures supply predicates and transformations.

- **Show real, small programs instead of vague claims**  
  - Walk an interviewer through one micro‑project.  
  - Point to specific code and explain what invariants Rust is enforcing for you.

If you work through each project and can explain it in your own words, you will have **strong, concrete proof** that you can reason about **systems design, memory‑safe programming, and reusable type-safe abstractions in Rust**.

## Prerequisites

- Rust toolchain installed: `rustc --version` and `cargo --version` should both work.  
  Install from the official site using `rustup` if needed.

Once that’s set up, everything else you need is already in this repo. Open a folder, run `cargo run`, and learn Rust basics along with me.