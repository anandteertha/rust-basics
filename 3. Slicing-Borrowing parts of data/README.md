# Project 003: Slices - Borrowing Parts of Data

This small Rust project explores one of the most important ideas in Rust: **slices**.

A slice is a borrowed view into existing data. It does not own the data. It simply allows us to access a portion of it safely and efficiently.

In this project, I worked with:

- string slices using `&str`
- array slices using `&[T]`
- helper functions that return borrowed parts of a string
- manual slicing with ranges

## What this project demonstrates

The program:

- finds the first word of a sentence
- finds the last word of a sentence
- borrows a specific substring using index ranges
- borrows part of an array using slice syntax

## Sample output

```text
main: first word of `rust makes borrowing explicit` is `rust`
main: last word of `rust makes borrowing explicit` is `explicit`
main: random slice of `rust makes borrowing explicit` from 5..10 is `makes`
main: middle two elements of `[1, 2, 3, 4]` is `[2, 3]`
```