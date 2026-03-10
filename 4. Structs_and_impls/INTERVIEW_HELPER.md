
## `interview-helper.md`

```markdown
# Interview Helper - Project 004: Structs + `impl`

This guide is meant to help you explain this project confidently in interviews and deeply understand the Rust concepts behind it.

---

# 1. One-line summary

I built a small Rust project called **Connection Tracker** to learn how to model state using a `struct` and attach behavior using an `impl` block. It also helped me understand the difference between associated functions and methods, and when to use `&self` versus `&mut self`.

---

# 2. What this project demonstrates

This project demonstrates:
- defining a custom type with `struct`
- grouping related fields into one entity
- implementing associated functions and methods
- using mutable and immutable borrowing in methods
- state-based logic inside methods
- encapsulating behavior rather than directly changing fields from outside

---

# 3. Core Rust concepts you should know

## Struct
A `struct` lets you group multiple related values together into a single custom type.

In this project, the `Connection` struct stores:
- id
- address
- active state
- retry count

That makes the program easier to reason about because all connection-related data lives in one place.

---

## Impl block
An `impl` block defines behavior for a type.

In this project, `impl Connection` contains:
- `new(...)`
- `activate(...)`
- `deactivate(...)`
- `increment_retries(...)`
- `print_summary(...)`

So the `Connection` type is not just data. It also knows how to behave.

---

## Associated function
An associated function belongs to the type itself, not to a particular instance.

Example:
`Connection::new(...)`

You use it to create a new value.

This is similar to a constructor conceptually, though Rust does not have constructors built into the language in the same way as some OOP languages.

---

## Method
A method operates on an existing instance.

Example:
`conn.activate()`

A method always has a receiver like:
- `&self`
- `&mut self`
- `self`

---

## `&self`
Use `&self` when the method only needs to read the value and not change it.

Example:
`print_summary(&self)`

This borrows the instance immutably.

---

## `&mut self`
Use `&mut self` when the method needs to modify the instance.

Examples:
- `activate(&mut self)`
- `deactivate(&mut self)`
- `increment_retries(&mut self)`

This borrows the instance mutably.

---

# 4. Mental model for interviews

A clean way to explain this project:

- The `struct` defines what a connection is
- The `impl` block defines what a connection can do
- `new()` creates a connection
- methods like `activate()` and `increment_retries()` change its state
- methods using `&self` only inspect state
- methods using `&mut self` mutate state

That is the exact conceptual foundation interviewers want to hear.

---

# 5. Why this project is useful for learning Rust

In many languages, people learn objects through classes.

Rust does not use classes.

Instead, Rust separates:
- data definition via `struct`
- behavior via `impl`

That makes the design more explicit.

This project helps you internalize that model with a very small example.

---

# 6. Explain your project in 30 seconds

I created a simple Rust project called Connection Tracker. It uses a `Connection` struct to store connection-related fields like id, address, active status, and retry count. Then I used an `impl` block to define methods that change or inspect that state, such as activating the connection, deactivating it, incrementing retries, and printing a summary. The project helped me understand associated functions, methods, and the difference between immutable and mutable borrowing in Rust.

---

# 7. Explain your project in 60 seconds

I built a small Rust project called Connection Tracker to practice how Rust models state and behavior without classes. I defined a `Connection` struct that stores fields like id, address, whether the connection is active, and the retry count. Then I used an `impl` block to add behavior to that struct, including an associated function `new()` to create a connection and instance methods like `activate()`, `deactivate()`, and `increment_retries()` to mutate state.

One useful part of the project is that retry increments are only allowed when the connection is active, so the methods are not just changing fields blindly — they are enforcing state-aware behavior. This project helped me understand the role of `&self` and `&mut self`, as well as the difference between associated functions and methods in idiomatic Rust.

---

# 8. Likely interview questions and strong answers

## Q1. Why did you use a `struct` here?
Because I wanted to group related connection data into one coherent type. Instead of managing separate variables for id, address, status, and retries, the struct lets me model them as one domain entity.

---

## Q2. Why use an `impl` block?
An `impl` block is where Rust attaches behavior to a type. It keeps the operations related to `Connection` close to the data definition and makes the code easier to understand and maintain.

---

## Q3. What is the difference between `new()` and `activate()`?
`new()` is an associated function because it creates a new `Connection` and is called on the type itself. `activate()` is a method because it acts on an existing instance.

---

## Q4. Why does `activate()` take `&mut self`?
Because it changes the internal state of the connection by setting the active flag to true. Any method that mutates the instance needs mutable access.

---

## Q5. Why does `print_summary()` take `&self`?
Because it only reads the state and does not modify anything. Immutable borrowing is enough for inspection.

---

## Q6. Why is retry increment blocked when inactive?
That was a deliberate design choice to make the behavior state-aware. It shows that the struct is not just a passive container — it can enforce rules based on the current state.

---

## Q7. Why not make the fields public?
Keeping fields private is better for encapsulation. It prevents arbitrary external mutation and forces interaction through methods, which makes it easier to enforce rules.

---

## Q8. Why did you derive `Debug`?
I derived `Debug` so I could print the full struct state using `{:?}` for learning and inspection. It is convenient for small projects and debugging.

---

## Q9. What would you improve in this project?
I would likely:
- rename `active_status` to `active`
- rename `retry_count` to `retries`
- make `print_summary()` return a `String` instead of printing directly
- possibly replace the boolean active flag with an enum for richer states
- think about handling retry overflow more carefully

---

## Q10. What is the idiomatic way to call methods in Rust?
The idiomatic style is method syntax like `conn.activate()`. Rust also allows the fully qualified form like `Connection::activate(&mut conn)`, but method syntax is preferred for readability.

---

# 9. Important distinction: associated function vs method

## Associated function
Called on the type:
`Connection::new(...)`

Use this when creating or otherwise working at the type level.

## Method
Called on an instance:
`conn.activate()`

Use this when the behavior applies to a specific value.

This distinction is extremely common in interviews.

---

# 10. Important distinction: immutable vs mutable borrow in methods

## Immutable borrow
`&self`

The method can read the value but not change it.

## Mutable borrow
`&mut self`

The method can modify the value.

In your project:
- `print_summary` -> immutable borrow
- `activate` -> mutable borrow
- `deactivate` -> mutable borrow
- `increment_retries` -> mutable borrow

If an interviewer asks why this matters, the answer is:
because Rust enforces safe access patterns at compile time.

---

# 11. Common mistakes interviewers may test

## Mistake 1: thinking Rust has classes
Rust does not have classes. The closest pattern is combining `struct` with `impl`.

## Mistake 2: not understanding why `new()` is not a method
`new()` does not act on an existing instance, so it is not a method on `self`.

## Mistake 3: using `&mut self` when only reading
If the method only inspects state, `&self` is sufficient and more correct.

## Mistake 4: making all fields public immediately
That weakens encapsulation and makes state validation harder.

## Mistake 5: treating a struct as only storage
A good Rust model often combines data with constrained behavior through methods.

---

# 12. How to talk about the state rule elegantly

A good phrase to use:

> I wanted the `Connection` type to enforce valid behavior, not just hold data, so I made retry increments conditional on the connection being active.

That sounds strong in interviews because it shows design thinking, not just syntax knowledge.

---

# 13. What this project does not cover yet

Be clear about what is intentionally out of scope:
- enums
- traits
- generics
- lifetimes
- error types like `Result`
- pattern matching
- ownership transfer through `self`

That is fine. This project is focused on the fundamentals of `struct` and `impl`.

---

# 14. Follow-up topics interviewers may branch into

Once you explain this project, an interviewer might ask about:

## Enums
A better way to model more than two states

## Ownership
What happens if a method takes `self` instead of `&self` or `&mut self`

## Encapsulation
Why private fields are useful

## Formatting traits
Difference between `Debug` and `Display`

## Error handling
Whether methods should print directly or return something

You do not need to overcomplicate the current project, but you should be aware of these directions.

---

# 15. Great discussion points if you want to sound strong

You can say things like:
- I used a struct because the fields represent one coherent domain concept
- I kept behavior in an impl block to keep the type self-contained
- I used `&mut self` for state transitions and `&self` for inspection
- I added a guard so retries only happen when the connection is active
- this project helped me understand Rust’s non-class-based approach to modeling data and behavior

---

# 16. Mini self-test

Try answering these without looking:

1. Why is `new()` an associated function and not a method?
2. Why does `activate()` require `&mut self`?
3. Why can `print_summary()` use `&self`?
4. What does `derive(Debug)` enable?
5. Why is `conn.activate()` more idiomatic than `Connection::activate(&mut conn)`?
6. Why is keeping fields private often better?
7. What design rule did `increment_retries()` enforce?

If you can answer those smoothly, you understand the project well.

---

# 17. Strong closing takeaway

The biggest lesson from this project is that Rust models software design explicitly:

- `struct` defines the data
- `impl` defines the behavior
- `&self` reads
- `&mut self` mutates
- methods can enforce valid state transitions

That is a foundational Rust idea, and this project is a clean first step into it.