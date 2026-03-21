# Project 010 - Collections and File I/O

A small Rust project that reads incident logs from a file, parses them into strongly typed data, stores them in collections, aggregates them into summaries, and prints useful operational insights.

This project is where many earlier Rust basics finally start working together in one realistic flow:

**file input -> parsing -> validation -> owned structs -> collections -> aggregation -> filtered reporting**

Instead of learning one concept in isolation, this project combines several of them into one small but meaningful system.

## What this project does

The program reads incident records from a text file. Each line represents one incident in a pipe-separated format like this:

`critical|database|unresolved|Primary DB connection timeout`

Each valid line is parsed into a strongly typed `Incident` struct with:

- `Severity`
- `service`
- `Status`
- `message`

After loading the file, the program prints:

- how many lines were parsed successfully
- how many lines were skipped
- summary by severity
- summary by status
- summary by service
- unresolved incident messages

## Why this project matters

This project is an important milestone because it stops being just syntax practice.

Earlier projects focused on one Rust concept at a time. This one uses many of them together in a way that feels closer to a real backend utility or internal tooling script.

It teaches how to:

- read text data from a file
- validate and parse input safely
- convert raw text into enums and structs
- store data in `Vec`
- aggregate using `HashMap`
- build reusable generic helper functions
- filter records using iterators

## Concepts practiced

### 1. File I/O

The project reads incident data from a file and handles file open or read failures explicitly through `Result`.

This reinforces that file operations can fail and must be handled safely.

### 2. Parsing and validation

Each line is split using `|`, validated for the correct number of fields, trimmed, and converted into typed data.

Invalid lines are rejected with clear error messages instead of silently creating broken objects.

### 3. Structs and enums

Raw strings are converted into domain-specific enums:

- `Severity`
- `Status`

This makes the rest of the program safer and clearer than working with plain strings everywhere.

### 4. Collections

The valid incidents are stored in:

- `Vec<Incident>`

Aggregated summaries are stored in:

- `HashMap<Severity, usize>`
- `HashMap<Status, usize>`
- `HashMap<String, usize>`

### 5. Generics

A reusable generic counting helper is used to avoid duplicating logic across different summary types.

This helper accepts a closure that extracts the grouping key from each incident.

### 6. Iterators and filtering

The program uses iterator-based filtering to print unresolved incident messages.

This directly builds on the iterator and closure concepts from the previous project.

## Project structure

~~~text
10. Collections_and_File_IO/
├── Cargo.toml
├── incidents.txt
└── src/
    ├── main.rs
    ├── file_helper.rs
    ├── incident.rs
    ├── severity.rs
    └── status.rs
~~~

## Input format

Each incident is stored on one line in the following format:

`severity|service|status|message`

Example:

~~~text
critical|database|unresolved|Primary DB connection timeout
warning|auth|resolved|Repeated invalid login attempts detected
critical|payments|resolved|Payment gateway timeout during checkout
info|cache|unresolved|Cache miss spike after deployment
warning|api|unresolved|Slow requests increasing latency
info|database|resolved|Replica caught up successfully
critical|auth|unresolved|JWT verification failures across region
warning|payments|resolved|Refund queue delay
~~~

## Sample output

~~~text
Parsed lines: 8
Skipped lines: 0

Summary for Severity:
Critical: 3
Warning: 3
Info: 2

Summary for Status:
Resolved: 4
Unresolved: 4

Summary for Service:
database: 2
auth: 2
payments: 2
cache: 1
api: 1

Unresolved log messages:
Primary DB connection timeout
Cache miss spike after deployment
Slow requests increasing latency
JWT verification failures across region
~~~

Note: since summaries are printed from a `HashMap`, the exact order may vary unless explicitly sorted.

## Key implementation ideas

### Parsing incidents

`Incident::new(line: &str) -> Result<Self, String>` validates the line before creating an `Incident`.

It checks:

- exactly 4 fields exist
- severity is valid
- service is not empty
- status is valid
- message is not empty

This keeps invalid data out of the main collection.

### Generic counting

Instead of writing separate counting logic for severity, status, and service, the project uses one reusable helper:

- `count_by_severity`
- `count_by_status`
- `count_by_service`

All three delegate to:

- `generic_count`

This is a good example of using generics only after duplication becomes obvious.

### Why `usize` is used for counts

Counts are stored as `usize` because it is the idiomatic type for collection sizes and counters in Rust.

Using `u8` would cap counts at 255, which is not a good default.

### Why `Status` and `Severity` are `Copy`

These enums are small and cheap to copy, so returning them by value is simpler and more ergonomic than returning references.

## What I learned

This project helped me understand that once data enters a Rust program, it is worth converting it into proper types as early as possible.

Some important takeaways:

- invalid input should be rejected at the boundary
- `Vec` is the natural owned collection for loaded records
- `HashMap` is perfect for aggregation and summaries
- the `entry` API is the idiomatic way to update counters
- generics become useful when multiple functions share the same pattern
- iterators become much more meaningful when working on real stored data

## Challenges I faced

### Designing parsing properly

At first it was tempting to create incidents with fallback defaults like `Unknown` or empty strings. That made the parser too permissive.

The better approach was to return `Result<Self, String>` and reject malformed lines early.

### Making counting reusable

The counting logic for severity, status, and service was almost identical.

That led to extracting a generic helper that accepts a closure for the grouping key.

### Handling owned vs borrowed data

Grouping by `service` required cloning the `String` into the `HashMap`, while grouping by `Severity` and `Status` was simpler because those enums are small `Copy` types.

This was a useful ownership design lesson.

## Possible future improvements

A few natural next steps for this project would be:

- sort summaries before printing
- use relative file paths for easier portability
- print line numbers for invalid input
- skip blank lines explicitly
- support CLI input for file path
- split reporting logic into separate modules
- define a custom error type instead of using `String`

## How to run

From inside the project folder:

~~~bash
cargo run
~~~

Make sure the input file exists and follows the expected pipe-separated format.

## Final takeaway

This project is a strong bridge between Rust basics and small real programs.

It shows how core Rust ideas such as enums, structs, `Result`, collections, generics, iterators, and ownership can all work together in a single practical pipeline.