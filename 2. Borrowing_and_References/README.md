# Borrowing and References (Rust)

This micro project is a tiny, visual way to understand borrowing in Rust.

Instead of moving ownership into functions, we will pass references and watch how the original value stays valid, while Rust enforces safe access rules.

## What you will learn
- Shared borrowing: `&T` lets you read without taking ownership
- Mutable borrowing: `&mut T` lets you modify with exclusive access
- Borrow scopes: when a borrow starts and when it ends
- The core rules:
  - Many `&T` at the same time is OK
  - Exactly one `&mut T` at a time is OK
  - `&T` and `&mut T` cannot overlap

## Project idea
Reuse the `Tracker` concept from the previous project, but focus on “lending” it to functions.

You should be able to:
1) Create a Tracker in `main`
2) Pass `&Tracker` into a function that reads and prints it
3) After the function returns, use the same Tracker again in `main`

Then repeat with mutation:
4) Create a mutable Tracker in `main`
5) Pass `&mut Tracker` into a function that changes something (name, counter, tag, anything)
6) After the function returns, use the Tracker again in `main` and confirm the change persisted

## Workflow
1) Start with the shared borrow path (`&T`)
   - Call a function that takes `&Tracker`
   - Print inside the function
   - Print again in `main` after the call
   - The key observation: the Tracker still belongs to `main`

2) Add “many readers”
   - Create two shared references at the same time
   - Use both, observe it is allowed

3) Add the mutable borrow path (`&mut T`)
   - Call a function that takes `&mut Tracker`
   - Make a visible change
   - Print after the call

4) Trigger the classic error on purpose (learning moment)
   - Try to use the Tracker while it is mutably borrowed
   - Or try to create a shared borrow while a mutable borrow exists
   - Read the compiler message, then fix it by tightening the scope

## Expected learning checkpoints
You should be able to explain:
- Why `&Tracker` does not move ownership
- Why `&mut Tracker` requires exclusivity
- What “borrow ends” means in practical terms
- Why Rust forces these rules at compile time

## Common compiler messages you will see (and should embrace)
- “cannot borrow `x` as mutable because it is also borrowed as immutable”
- “cannot borrow `x` as immutable because it is also borrowed as mutable”
- “cannot use `x` because it was mutably borrowed”

These are not random errors. They are Rust pointing at overlapping access.

## Done when
- You have a clean run that demonstrates shared borrow and mutable borrow behavior
- You have at least one intentionally triggered borrow-checker error that you fixed by adjusting scope
- You can explain the rules in your own words