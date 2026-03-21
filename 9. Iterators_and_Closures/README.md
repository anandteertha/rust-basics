# 009_Iterators_and_Closures

A tiny Rust micro-project that demonstrates how iterator methods and closures work together to process structured alert data.

This project uses a small alert pipeline to show how Rust can read, filter, transform, search, check, and aggregate data in an expressive and safe way.

## What this project covers

This project demonstrates:

- `iter()`
- `filter()`
- `map()`
- `find()`
- `any()`
- `all()`
- `fold()`
- closures used with iterator methods

## Why this project matters

Until this point, the learning track focused heavily on ownership, borrowing, enums, structs, traits, generics, and lifetimes.

This project is where Rust starts to feel more expressive.

Instead of writing manual loops, temporary flags, and repeated push logic, iterators let us describe the operation we want:

- read all items
- keep only matching items
- transform items into another form
- search for one matching item
- check conditions across the collection
- reduce everything into one final value

Closures make this possible by letting us pass behavior directly into iterator methods.

## Project structure

- `main.rs`  
  Runs all iterator examples on a small alert dataset.

- `alert.rs`  
  Defines the `Alert` struct, related helper methods, and mock alert data.

- `alert_level.rs`  
  Defines the `AlertLevel` enum.

- `README.md`  
  Explains the project and the learning goals.

- `INTERVIEW_HELPER.md`  
  Contains the most important interview-ready concepts from this project.

## Data model

The project uses a small structured alert model:

- service name
- alert level
- message
- resolved status
- severity score

This keeps the examples realistic and slightly system-design flavored.

## Concepts demonstrated

### 1. `iter()`

`iter()` gives shared references to each item in the collection.

That means we can read the data without taking ownership of it.

In this project, `iter()` is used to print all alerts and to drive all the later iterator chains.

Core idea:

- `iter()` yields `&Alert`
- the original vector remains usable afterward

### 2. `filter()`

`filter()` keeps only the items that match a condition.

In this project, it is used to keep only critical alerts.

The closure passed into `filter()` must return a `bool`.

Core idea:

- `true` means keep the item
- `false` means discard it

### 3. `map()`

`map()` transforms each item into another form.

In this project, each alert is converted into a summary string.

Core idea:

- `map()` changes the shape of the data
- it does not decide whether to keep or discard items

### 4. `find()`

`find()` searches for the first item that matches a condition.

In this project, it is used to find the first unresolved critical alert.

Core idea:

- `find()` stops early once a match is found
- it returns an `Option`

### 5. `any()` and `all()`

These methods express boolean checks over a collection.

In this project:

- `any()` checks whether any alert is critical
- `all()` checks whether all alerts are resolved

Core idea:

- `any()` returns `true` if at least one item matches
- `all()` returns `true` only if every item matches

### 6. `fold()`

`fold()` reduces the entire iterator into one accumulated value.

In this project, it is used to compute the total severity score across all alerts.

Core idea:

- start with an initial accumulator
- update it for every item
- return the final accumulated result

## Closures in this project

Closures are used throughout this project with iterator methods.

Examples include:

- checking whether an alert is critical
- converting alerts into summary strings
- checking resolved state
- accumulating severity scores

This project intentionally uses closures in context rather than teaching them in isolation.

That makes the learning more practical, because closures are most commonly seen when working with iterators.

## Important learning outcomes

After this project, you should be comfortable with:

- reading data through iterators
- understanding that iterator methods are lazy until consumed
- knowing that `filter()` and `find()` need predicate closures returning `bool`
- recognizing that `map()` transforms items
- seeing how `fold()` builds one final result
- understanding that closures are small pieces of behavior passed into iterator adapters

## Sample output

The program prints:

- all alerts
- only critical alerts
- mapped alert summaries
- the first unresolved critical alert
- boolean checks using `any()` and `all()`
- the total severity score using `fold()`

This makes each iterator method visible and easy to verify.

## Key Rust takeaways

### Iterators are lazy

Iterator adapters such as `filter()` and `map()` do not do any work by themselves.

They only run when the iterator is consumed by something like:

- a `for` loop
- `find()`
- `any()`
- `all()`
- `fold()`
- `collect()`

### Borrowing still matters

When using `.iter()`, the iterator yields borrowed items.

That means the collection must live long enough for those references to remain valid.

### Closures must match the method’s expectation

For example:

- `filter()` expects a closure returning `bool`
- `find()` expects a closure returning `bool`
- `map()` expects a closure returning a transformed value
- `fold()` expects a closure that returns the next accumulator value

## Why this project is useful for interviews

This project helps build intuition for several common early Rust interview questions:

- What is the difference between `map()` and `filter()`?
- Why does `filter()` need a boolean predicate?
- What does `find()` return?
- What does `fold()` do?
- Why are iterators considered idiomatic in Rust?
- How are closures used with iterator methods?
- What does `.iter()` yield?

## Final note

This project is an important step because it moves Rust from feeling rule-heavy to feeling expressive.

Ownership and borrowing are still present, but now they support a cleaner and more declarative way of processing data.

That is one of the biggest mindset shifts in learning Rust.