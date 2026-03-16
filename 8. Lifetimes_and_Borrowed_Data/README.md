# 8. Lifetimes and Borrowed Data

A small Rust project that demonstrates how **lifetimes** work when structs store **borrowed string slices** instead of owned `String` values.

This project parses simple service status logs like:

database:connected  
cache:degraded  
auth:disconnected  

and converts each line into a `LogEntry` struct that **borrows** the service name from the original input.

## Why this project exists

Up to this point, Rust fundamentals usually cover:

- ownership
- borrowing
- slices
- structs and impl
- enums and match
- traits and generics

The next important step is understanding what happens when a struct itself stores references.

That is where lifetimes become necessary.

This project helps make that idea concrete.

## What this project teaches

- why structs with references need lifetime parameters
- how borrowed `&str` data can be stored safely inside a struct
- how a parser can return data tied to the lifetime of its input
- how a `Vec<LogEntry<'a>>` depends on the source input staying alive
- how Rust prevents dangling references at compile time

## Project idea

The program parses a multiline string of system status logs and builds structured entries for each line.

Each log line follows this format:

service:status

Example:

- database:connected
- cache:degraded
- auth:disconnected

Each parsed line becomes a `LogEntry` containing:

- the service name as a borrowed `&str`
- the log status as a `LogType` enum

## Core data model

### `LogType`
Represents the parsed health state of a service.

Possible values:

- `Connected`
- `Degraded`
- `Disconnected`
- `Unknown`

### `LogEntry<'a>`
Represents one parsed service log entry.

It stores:

- `service: &'a str`
- `log_type: LogType`

The lifetime `'a` means the `service` field is borrowed from some input that must outlive the `LogEntry`.

## Why the lifetime matters

The `service` field does not own data.

It points into the original input string.

That means a `LogEntry` cannot live longer than the source text it was parsed from.

Rust uses the lifetime parameter on `LogEntry<'a>` to enforce that relationship.

## Example flow

1. Create a multiline input string
2. Iterate through each line
3. Parse each line into a `LogEntry`
4. Store entries in a vector
5. Print:
   - all entries
   - problematic entries
   - severe entries

## Example output

```text
Entries
service=database, status=Connected
service=cache, status=Degraded
service=auth, status=Disconnected
service=payments, status=Connected
service=search, status=Degraded

problematic entries:
service=cache, status=Degraded
service=auth, status=Disconnected
service=search, status=Degraded

severe entries:
service=auth, status=Disconnected
```

## Project structure

```text
src/
├── main.rs
└── log_entry.rs
```

### `main.rs`

Responsible for:

- providing the input string
- calling `LogEntry::parse(...)`
- collecting parsed entries
- printing different filtered views

### `log_entry.rs`

Responsible for:

- defining `LogType`
- defining `LogEntry<'a>`
- implementing parser and helper methods
- mapping raw status strings into enum values

## Parsing strategy

Each line is parsed using `split_once(":")`.

If the line is valid:

- left side becomes the borrowed service name
- right side becomes a `LogType`

If the line is invalid:

- parsing returns an error

This keeps the logic small and focused while still demonstrating borrowed return values.

## Key Rust idea shown here

The important method shape in this project is:

- input: borrowed `&str`
- output: `LogEntry` that borrows from that input

That is the reason lifetimes matter here.

The parser is not creating fresh owned strings.  
It is returning structured borrowed views into existing text.

## Why this is a good Rust learning project

This project is small enough to understand quickly, but it teaches one of the most important Rust concepts clearly:

**references inside structs must be tied to valid input lifetimes.**

That makes lifetimes feel practical instead of abstract.

## Possible improvements

Future extensions could include:

- parsing all lines through a separate helper function
- adding a line number to each `LogEntry`
- computing the most severe service automatically
- summarizing counts of connected/degraded/disconnected services
- parsing configuration key-value pairs using the same borrowed-data model

## Takeaway

This project demonstrates that lifetimes are not about manually managing memory.

They are about expressing a relationship:

**this borrowed data inside my struct came from that input, so the struct must not outlive the input.**

That is the heart of Rust lifetimes.