# Traits and Generics in Rust

A small Rust project that demonstrates how to model shared behavior using traits and process multiple concrete types using generics.

## Project Overview

This project simulates a simple connection monitoring system with three different connection types:

- `DatabaseConnection`
- `ApiConnection`
- `CacheConnection`

Each connection type stores:

- a name
- a status
- its own type-specific metadata

Although these are different concrete types, they all implement the same `Connection` trait. A generic function then processes any type that implements this trait and prints a summary.

This project focuses on one of the most important transitions in Rust: moving from writing logic for one concrete type to writing logic for any type that satisfies a shared behavior contract.

## Learning Objective

The goal of this project is to understand:

- how traits define shared behavior across multiple types
- how generics allow reusable logic across different concrete types
- how trait bounds make generic functions type-safe
- how to design around behavior instead of hardcoding branches everywhere

## Core Rust Concepts Covered

- Traits
- Generics
- Trait bounds
- Structs
- Enums
- `impl`
- `match`
- Basic parsing with `split_whitespace()`
- Method dispatch through shared behavior

## Project Story

Imagine a small monitoring system that receives logs about different service connections.

Some logs describe:

- a database connection
- an API connection
- a cache connection

Each connection has its own metadata and internal details, but from the outside, all of them should support a common monitoring interface.

This project models that idea by defining a shared `Connection` trait and implementing it for all three connection types.

## Sample Input

```text
database primary_db connected
api payments_api degraded
cache redis_cache connected
database analytics_db disconnected
api auth_api connected
cache session_cache degraded
```