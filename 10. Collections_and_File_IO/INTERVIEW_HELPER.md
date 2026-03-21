# Interview Helper - Project 010: Collections and File I/O

Use this document to explain the project clearly in interviews, discussions, or while writing about it.

## 1. Project summary in one line

I built a small Rust tool that reads incident logs from a file, parses them into strongly typed structs and enums, stores them in collections, aggregates them using `HashMap`, and prints operational summaries and unresolved incident messages.

## 2. Slightly longer version

This project was my first end-to-end Rust data pipeline.

Instead of hardcoding data, I read incident records from a file, validated each line, converted it into a typed `Incident` struct, stored valid records in a `Vec`, and then produced summaries such as counts by severity, status, and service using `HashMap`.

I also extracted the repeated counting logic into a generic helper that accepts a closure to choose the grouping key.

## 3. Why I built this project

I wanted a project that felt more real than isolated concept demos.

Earlier Rust projects taught ownership, enums, `Result`, traits, lifetimes, and iterators separately. This project was about using many of those ideas together in one practical workflow:

- file input
- safe parsing
- validation
- owned data
- collections
- aggregation
- filtering

So this project became a capstone-style basics project.

## 4. Core Rust concepts demonstrated

### File I/O

I read the incident data from a file and handled file open or read failure explicitly through `Result`.

### Parsing with validation

Each line is split by `|`, trimmed, validated, and converted into an `Incident`.

This avoids letting malformed input silently enter the system.

### Enums

I used enums for:

- `Severity`
- `Status`

This gave the project stronger type safety than plain strings.

### Structs

Each parsed record becomes an `Incident` struct.

That makes the data easier to work with and easier to aggregate.

### Collections

I used:

- `Vec<Incident>` to store all valid records
- `HashMap<K, usize>` to count grouped summaries

### Generics

I noticed that counting by severity, status, and service all followed the same pattern, so I extracted that repeated logic into a generic helper.

### Iterators and closures

I used iterators and closures to filter unresolved incidents and to build a reusable counting abstraction.

## 5. Design choices I can explain

### Why use enums instead of strings?

Enums make the domain explicit and prevent invalid values from spreading throughout the system.

For example, once parsing succeeds, the rest of the code works with `Severity::Critical` instead of a raw `"critical"` string.

### Why return `Result` from parsing?

Because not every input line is valid.

Returning `Result<Self, String>` makes parsing explicit, keeps invalid data out of the main collection, and lets the caller decide how to handle failures.

### Why store incidents in a `Vec`?

A `Vec` is the natural owned collection for a loaded list of records.

It keeps the valid parsed incidents in memory so they can be summarized and filtered multiple times.

### Why use `HashMap` for summaries?

Because it is the natural data structure for counting grouped data.

It lets me efficiently aggregate by severity, status, or service.

### Why use `usize` for counts?

`usize` is the idiomatic Rust type for counts and collection sizes.

It is more appropriate than smaller integer types like `u8`.

### Why use a generic counting helper?

Because the counting logic was almost identical across multiple summary functions.

Instead of duplicating loops, I built one helper that accepts a closure to extract the grouping key.

## 6. The generic helper idea

A very important idea in this project was this:

Instead of writing separate counting loops for every category, I wrote one generic function that answers:

"Given a list of incidents and a way to extract a key from each incident, how many times does each key appear?"

This helper:

- takes a slice of incidents
- accepts a closure like `|incident| incident.severity`
- returns a `HashMap` of counts

That made it reusable for:

- severity
- status
- service

## 7. Why `entry()` is idiomatic here

For counting in a `HashMap`, Rust’s `entry` API is the idiomatic tool.

It avoids doing repeated lookups like:

- `contains_key`
- `get`
- `insert`

Instead, it handles "update if present, initialize if absent" in one place.

That makes the counting logic cleaner and more efficient.

## 8. Ownership and borrowing lessons from this project

This project reinforced several important ownership lessons.

### `Severity` and `Status`

These are small enums, so they can be `Copy`.

That makes them easy to use as keys and easy to return by value.

### `service`

`service` is a `String`, so grouping by service required cloning it when inserting into the `HashMap`.

That was a good example of how owned heap data behaves differently from small copyable enums.

### Borrowed accessors

It made sense to return `&str` for message access because cloning the whole `String` just for printing would be unnecessary.

## 9. Challenges I faced and how I solved them

### Challenge 1: The parser was too permissive at first

Initially, it was possible to create incidents with fallback defaults like `Unknown` or empty strings.

That made invalid input look valid.

**Fix:** I changed the parser to return `Result<Self, String>` and reject malformed lines early.

### Challenge 2: Repeated aggregation logic

Counting by severity, status, and service all looked similar.

**Fix:** I extracted a generic helper with a closure-based key selector.

### Challenge 3: Thinking about owned vs borrowed map keys

Grouping by enums was easy because they are small and copyable. Grouping by service required cloning strings.

**Fix:** I accepted cloning for service names because it kept the design simple and correct for this project.

## 10. Good interview explanation of the project flow

The program works in five stages:

1. read raw incident text from a file
2. split each line and validate its shape
3. convert valid lines into typed `Incident` values
4. store valid incidents in a `Vec`
5. aggregate and filter them to produce summaries and unresolved incident output

That lets me talk about the project as a full pipeline rather than as a collection of unrelated functions.

## 11. Common interview questions and how to answer them

### Q: Why not just use strings everywhere?

Because enums make invalid states harder to represent and make the code easier to reason about after parsing.

### Q: Why use `Result` instead of silently skipping bad fields?

Because parsing is a boundary operation. Invalid input should be handled explicitly rather than creating partially valid objects.

### Q: Why extract generics here?

Because the repeated logic became obvious across multiple functions, and a generic helper reduced duplication while keeping the public API readable.

### Q: Why not return references from everything?

Because references are most useful when avoiding expensive copies of larger data. For small `Copy` enums, returning values is simpler and more ergonomic.

### Q: Why `HashMap` instead of a vector of counters?

Because service names are dynamic keys, and even enum-based grouping fits naturally into key-value aggregation logic.

### Q: What would you improve next?

I would add stable sorting for summaries, better error types, line-number-aware parse errors, relative file path support, and cleaner report formatting.

## 12. Strong concise answer for "What did you learn?"

I learned how Rust’s type system becomes really valuable once raw external input enters the program. By validating early, converting strings into enums, storing valid records in owned collections, and using generic helpers for repeated aggregation patterns, I could keep the rest of the code much safer and cleaner.

## 13. Strong concise answer for "What was the most Rust-y part?"

The most Rust-y part was using `Result` to make parsing explicit and using a generic helper with a closure-based key extractor to avoid duplicated counting logic. That, along with `HashMap::entry`, made the aggregation both safe and idiomatic.

## 14. Strong concise answer for "Why is this project valuable?"

Because it is not just about syntax. It shows how Rust concepts like structs, enums, `Result`, collections, generics, ownership, and iterators come together to build a realistic little data-processing tool.

## 15. Final takeaway

This project helped bridge the gap between learning Rust features and building something real in Rust.

It made file input, validation, ownership, collections, generic abstraction, and iterator-based filtering all work together in one coherent program.