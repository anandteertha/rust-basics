
# Project 005 - Interview Helper

## One-line summary

This project models a connection lifecycle in Rust using enums and `match`, making state transitions explicit and easy to reason about.

## Strong short explanation

I built a small Rust program that simulates the lifecycle of a connection object. Instead of storing status as plain strings, I used an enum to represent states like `Disconnected`, `Connecting`, `Connected`, and `Failed`. Then I used `match` inside methods to control transitions and handle different runtime scenarios cleanly.

## 30-second explanation

This project was about learning how Rust enums and pattern matching help model real system behavior. I created a `Connection` struct with fields like `id`, `address`, `active`, and `status`, where `status` is an enum. Some methods, like `start_connecting()` and `mark_connected()`, are state-aware and only work when the connection is in the right condition. Other methods, like `fail()` and `reset()`, are intentionally designed as force operations that can happen at any time. That helped me build a small but realistic state-driven system.

## 60-second explanation

In this project, I wanted to move beyond basic structs and start modeling behavior in a more Rust-oriented way. I created a `Connection` struct and a separate enum for connection state. The enum has four variants: `Disconnected`, `Connecting`, `Connected`, and `Failed`.

The key idea was to use `match` to make lifecycle transitions explicit. For example, `start_connecting()` only works if the connection is active, and `mark_connected()` only makes sense when the current state is `Connecting`. At the same time, I intentionally designed `fail()` and `reset()` as global override operations, because in real systems failures and resets can happen unexpectedly or externally at any point.

This project helped me think about Rust not just as a language for writing syntax, but as a way to model state clearly and safely through types.

## Core Rust concepts demonstrated

- enums
- pattern matching with `match`
- structs
- impl blocks
- ownership of `String`
- borrowing with `&str`
- method-based API design
- domain modeling through types

## Why enums are better here

Using an enum is better than using strings like `"connected"` or `"failed"` because:

- only valid states can exist
- the compiler helps enforce handling of all cases
- control flow becomes more explicit
- code is easier to maintain and reason about

## Why `match` is important here

`match` is useful because it makes branching based on state clear and exhaustive.

Benefits:

- every state is handled explicitly
- logic is easier to read than nested `if/else`
- invalid assumptions are harder to hide
- the code scales better when more states are added

## Important design decision

### Why keep both `active` and `status`?

Because they represent different ideas.

- `active` tells us whether the connection is enabled and allowed to begin work
- `status` tells us where the connection is in its lifecycle

That allows combinations like:

- active + failed
- inactive + disconnected
- active + connected

This makes the model a bit richer than collapsing everything into a single enum.

## Another important design choice

### Why can `fail()` and `reset()` happen at any time?

That was intentional.

I treated them as force operations rather than strictly controlled transitions.

Reasoning:

- a real connection can fail unexpectedly at any moment
- a reset can be triggered manually or by the system regardless of current state

So the design is:

- `start_connecting()` and `mark_connected()` are controlled, state-aware operations
- `fail()` and `reset()` are global override operations

That gives the project a realistic lifecycle model while still showing how enums and `match` work.

## Likely interview questions and answers

### 1. Why did you use an enum for status?

Because the connection can only be in a fixed set of valid states. Enums let me model that directly and avoid invalid string values. They also work well with `match`, which makes transitions explicit.

### 2. Why not use only a boolean field like `is_connected`?

A boolean would not capture the full lifecycle. I needed to distinguish between `Disconnected`, `Connecting`, `Connected`, and `Failed`, so an enum was a much better fit.

### 3. Why did you keep the `active` field if status already exists?

Because `active` and `status` mean different things. `active` tells whether the connection is enabled for work, while `status` describes the current lifecycle stage.

### 4. Why can `fail()` be called even after `Connected`?

Because I modeled `fail()` as a runtime failure event, not just a failed connection attempt. A live connection can drop or break unexpectedly, so allowing `Connected -> Failed` is realistic.

### 5. Why can `reset()` happen from any state?

Because reset is intended as a hard cleanup operation. It does not represent a normal lifecycle transition. It represents a forceful return to a clean baseline.

### 6. What does this project say about your design thinking?

It shows that I try to model behavior explicitly through types and state instead of keeping logic hidden in loosely structured flags or strings.

### 7. What would you improve next?

I would likely rename `State` to `ConnectionState`, add retry tracking, and potentially store a transition history so the lifecycle becomes even more observable.

## Good talking points for recruiters or interviewers

- modeled system state explicitly with enums
- used `match` for readable control flow
- separated operational enablement from lifecycle status
- handled both happy-path and edge-case scenarios
- designed force operations to reflect real-world system behavior

## What makes this project stronger than a toy example

It does not just print enum variants once.

It simulates multiple realistic flows:

- successful connect
- failed connection
- connect attempt while inactive
- reset after success
- runtime failure after connection

That makes the project more convincing because it tests actual behavior rather than just syntax.

## If asked "What did you learn?"

A strong answer:

I learned that Rust enums are not just a feature for grouping variants. They are a very clean way to model real system state. I also learned that `match` makes control flow much more intentional because every state has to be considered explicitly. This project helped me think more in terms of state modeling and API behavior rather than just method implementation.

## If asked "What would you do differently?"

A strong answer:

I would probably rename `State` to `ConnectionState` for clarity, and I would make my logging more consistent by using the `status()` method everywhere instead of mixing it with debug formatting. I would also consider adding retry counts or event history if I wanted to expand the system.

## Resume-style project framing

Built a Rust-based connection lifecycle simulator using enums and pattern matching to model explicit state transitions such as disconnected, connecting, connected, and failed, with support for both guarded transitions and global override operations like reset and failure.

## Best takeaway line

This project shows how Rust types can be used to model behavior clearly, not just store data.