# Project 003 Interview Helper - Slices and String Slices

This document is meant to help revise the core ideas behind Rust slices for interviews, discussions, and self-review.

---

## 1. What is a slice in Rust?

A slice is a **borrowed view into a contiguous portion of data**.

It does not own the data.  
It only references part of data owned somewhere else.

Examples:
- `&str` -> string slice
- `&[T]` -> slice of elements of type `T`

---

## 2. Why are slices useful?

Slices are useful because they let us:
- borrow only the part of data we need
- avoid unnecessary copying or cloning
- write more flexible APIs
- express read-only access clearly

Instead of creating a new owned value, we can often just return a slice.

---

## 3. Difference between `String` and `&str`

### `String`
- owned
- heap allocated
- growable
- responsible for cleanup

### `&str`
- borrowed
- read-only view into string data
- not responsible for cleanup
- usually points to a `String` or a string literal

A good interview line:

> `String` owns text, while `&str` borrows text.

---

## 4. What is the relation between ownership and slices?

Ownership decides who owns the underlying data.

A slice borrows part of that owned data.

That means a slice:
- depends on the original owner staying alive
- cannot outlive the data it points to
- follows Rust borrowing rules

A good interview line:

> A slice is not independent data. Its validity depends on the original owned value.

---

## 5. What is a string slice?

A string slice is `&str`.

It is a borrowed view into valid UTF-8 text.

Examples:
- a string literal like `"hello"` has type `&str`
- a portion of a `String` can also be borrowed as `&str`

---

## 6. What is an array slice?

An array slice is `&[T]`.

It borrows a contiguous region of an array or vector without taking ownership.

Example idea:
- full array: `[1, 2, 3, 4]`
- slice: `[2, 3]`

A good interview line:

> Slicing is not specific to strings. It is a general Rust concept for borrowing contiguous data.

---

## 7. Why do many Rust functions prefer `&str` instead of `String`?

Because many functions only need to read text, not own it.

Using `&str` makes the function:
- more flexible
- more efficient
- less allocation-heavy

A function taking `&str` can accept:
- string literals
- borrowed `String`s
- other string slices

A good interview line:

> `&str` is often preferred in APIs because it accepts more inputs and avoids unnecessary ownership transfer.

---

## 8. Why return `&str` instead of `String`?

Return `&str` when:
- the result is just a borrowed part of input text
- you do not need a new owned string
- you want to avoid allocation

Return `String` when:
- you are creating new text
- the result must own its data
- the returned value cannot safely borrow from input

A good interview line:

> Return `&str` for borrowed output, return `String` for owned output.

---

## 9. What is the key safety rule with slices?

A slice must never outlive the data it refers to.

Rust enforces this at compile time.

This prevents dangling references.

A good interview line:

> Rust uses borrowing and lifetime rules to guarantee that slices never point to invalid memory.

---

## 10. Why can slicing strings be tricky in Rust?

Strings in Rust are UTF-8.

That means:
- one character may take more than one byte
- string indices are byte indices, not character positions
- slicing must happen on valid UTF-8 boundaries

This is why arbitrary string slicing can panic if indices do not align properly.

A good interview line:

> String slicing in Rust uses byte offsets, so indices must land on valid UTF-8 boundaries.

---

## 11. Why was `&owned_string[5..10]` valid in this project?

Because the chosen indices matched valid byte boundaries in plain ASCII text:
`rust makes borrowing explicit`

The slice `5..10` corresponds to `makes`.

This worked safely because ASCII characters are one byte each.

Important interview nuance:

> It works here because the text is ASCII. In general UTF-8 strings need more care.

---

## 12. Why is `split_whitespace()` often more idiomatic than manual byte scanning?

`split_whitespace()` is often more idiomatic because it:
- is clearer to read
- expresses intent directly
- avoids unnecessary manual indexing logic
- reduces boundary mistakes

Manual byte scanning is still useful for learning and for low-level understanding.

A good interview line:

> The iterator-based approach is usually more idiomatic, while byte scanning helps understand what slices are doing underneath.

---

## 13. What bug happened in the project and what did it teach?

The first implementation of getting the last word returned `borrowing` instead of `explicit`.

Why that happened:
- words were only pushed when a space was found
- the final word had no trailing space
- so it was never added

What it taught:
- boundary conditions matter a lot
- manual scanning requires explicit handling of the final segment
- slices are simple conceptually, but indexing logic can still be error-prone

This is a strong interview story because it shows debugging and understanding, not just syntax.

---

## 14. Why was using a `Vec<&str>` for the last word correct but not ideal?

It was correct because:
- each word slice borrowed from the original sentence
- the final `pop()` returned the last collected word

But it was not ideal because:
- it stored all words unnecessarily
- it did more work than needed
- only the final word was actually required

A better design would identify the last slice directly.

A good interview line:

> The solution was functionally correct, but not optimal because it allocated intermediate storage for a problem that only needed one final borrowed slice.

---

## 15. What does a slice actually contain conceptually?

Conceptually, a slice contains:
- a pointer to the start of the data
- a length

For string slices, it points to UTF-8 data.  
For array slices, it points to elements of a specific type.

You usually do not manipulate this directly, but it is useful interview knowledge.

---

## 16. Common interview questions and sharp answers

### Q: What is a slice?
A borrowed view into a contiguous portion of data.

### Q: Does a slice own data?
No. It only borrows data.

### Q: What is the difference between `String` and `&str`?
`String` owns heap-allocated text. `&str` is a borrowed view into text.

### Q: Can slices only be used with strings?
No. Arrays and vectors can also be sliced using `&[T]`.

### Q: Why use `&str` in function parameters?
Because it avoids ownership transfer and works with both string literals and `String`s.

### Q: Why can string slicing panic?
Because Rust strings are UTF-8 and indices must be valid byte boundaries.

### Q: What does Rust protect you from with slices?
Dangling references and invalid memory access.

---

## 17. Strong interviewer-facing explanation of this project

In this project, I explored slices as borrowed views into data rather than owned values. I worked with both string slices and array slices. The key learning was understanding the distinction between `String` and `&str`, and seeing how Rust allows us to return borrowed parts of data without cloning. I also debugged a boundary-condition bug while extracting the last word manually, which helped reinforce how slice logic depends on correctly handling the final segment of input.

---

## 18. What you should be able to explain confidently after this project

You should now be able to explain:
- what a slice is
- why slices do not own data
- why `&str` is common in Rust APIs
- how slices relate to borrowing and lifetimes
- why string slicing is byte-based
- why UTF-8 makes string indexing more careful than array indexing
- the difference between an idiomatic solution and a merely correct one

---

## 19. Best concise summary for revision

> A slice in Rust is a borrowed view into contiguous data.  
> `String` owns text, while `&str` borrows text.  
> Slices help avoid copying, but they depend on the original data remaining valid.  
> String slices must also respect UTF-8 byte boundaries.

---

## 20. If asked "what did this project really teach you?"

A strong answer:

> This project taught me that Rust is not just about ownership of whole values. It also gives very precise tools for borrowing only the part of data I need. Slices made that idea very practical for me, especially the distinction between `String` and `&str`, and the importance of correctness when working with boundaries.