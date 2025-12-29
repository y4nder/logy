# 🦀 Logy — A Rust-Based Log Analysis Prototype

![Rust](https://img.shields.io/badge/language-Rust-orange)
![CLI](https://img.shields.io/badge/type-CLI-blue)
![Learning Project](https://img.shields.io/badge/status-learning%20project-yellow)

## Overview

**Logy** is a lightweight command-line log analyzer implemented in **Rust**, developed as a learning-oriented prototype to explore Rust’s core language features and idiomatic programming patterns.

The project focuses on **fundamental systems programming concepts** such as structured data parsing, iterator-driven pipelines, explicit error handling, and strong type modeling.
While minimal in scope, Logy is intentionally designed to serve as a foundation for future experimentation and incremental expansion.

---

## Motivation

Rust is increasingly adopted in systems and infrastructure software due to its emphasis on **memory safety**, **predictable performance**, and **compile-time correctness guarantees**.
Log analysis represents a practical problem domain well-suited for studying these characteristics, as it involves:

* File I/O
* Sequential data processing
* Domain modeling with enums and structs
* Aggregation and filtering operations

Logy was created to study these aspects in isolation, without the complexity of production-scale tooling.

---

## Learning Objectives

This project is primarily intended to support learning and experimentation.
Key objectives include:

* Understanding Rust’s ownership and borrowing model in practice
* Modeling domain concepts using enums and strongly typed structs
* Applying iterator-based data transformation pipelines
* Handling invalid or partial data safely using `Option` and `Result`
* Writing clear, maintainable, and idiomatic Rust code

---

## Current Capabilities

Logy currently supports:

* Reading log files line-by-line
* Parsing a fixed, structured log format
* Representing log severity using a strongly typed enum
* Counting log entries by severity level
* Optional filtering by log level via command-line arguments
* CLI-based output suitable for further extension

### Supported Log Format

```
YYYY-MM-DD LEVEL Message
```

Example:

```
2025-03-21 INFO User login succeeded
2025-03-21 WARN Rate limit approaching
2025-03-21 ERROR Database timeout
```

---

## Project Structure

The project is intentionally kept minimal to emphasize clarity and learning:

```
logy/
├── src/
│   ├── main.rs        # Program entry point and processing pipeline
│   ├── log.rs         # Domain models (LogLevel, LogEntry)
│   ├── parser.rs     # Log line parsing logic
│   └── analyzer.rs   # Aggregation and analysis routines
└── Cargo.toml
```

---

## Usage

### Build and run

```bash
cargo run -- app.log
```

### Filter by log level

```bash
cargo run -- app.log --level=ERROR
```

### Build optimized binary

```bash
cargo build --release
```

---

## Design Philosophy

Logy prioritizes:

* Explicitness over convenience
* Strong typing over string-based representations
* Simplicity over premature abstraction
* Learning value over feature completeness

As such, the project deliberately avoids advanced features (e.g., async I/O, concurrency, external frameworks) until foundational understanding is established.

---

## Future Work

As a personal hobby and learning project, Logy may be expanded incrementally to explore more advanced Rust concepts, such as:

* Improved CLI ergonomics
* Structured output formats (e.g., JSON)
* Parallel or streaming log processing
* Performance benchmarking and profiling
* Support for additional log schemas

These enhancements will be guided by continued study and experimentation with Rust.

---

## Disclaimer

This project is **not intended for production use**.
It is a learning artifact designed to document progress in understanding Rust and systems programming concepts through hands-on implementation.

---

## Author’s Note

Logy represents an early step in a longer-term exploration of Rust.
The project will evolve organically as understanding deepens, and it serves both as a technical exercise and a reference point for future work.
