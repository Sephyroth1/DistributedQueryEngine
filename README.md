# Distributed Query Engine (DB-Lite)

## Overview

This project is a **from-scratch distributed query engine** designed to explore core concepts in:

* Systems programming
* Query execution
* Distributed systems
* Storage engines
* Virtual machines
* Compiler-like parsing

The goal is not to build a production database, but to deeply understand how such systems work by implementing them end-to-end.

---

## Goals

* Build a working query engine capable of:

  * Parsing queries
  * Generating execution plans
  * Executing queries locally and across nodes
* Design a modular system with clear separation of concerns
* Explore trade-offs in performance, consistency, and architecture
* Provide observability via a web-based interface

---

## Non-Goals

* Full SQL compliance
* Production-grade fault tolerance
* High performance optimizations (initially)
* Feature completeness over understanding

---

## High-Level Architecture

The system is divided into the following layers:

### 1. Query Layer (Parser & Planner)

* Tokenizes input queries
* Builds an Abstract Syntax Tree (AST)
* Converts AST into an execution plan

### 2. Execution Engine (VM)

* Executes query plans using a custom instruction set
* Stack-based virtual machine
* Supports basic operations (scan, filter, project)

### 3. Storage Engine

* Persistent key-value store
* Basic indexing (planned: B-Tree or LSM)
* Handles serialization/deserialization

### 4. Distributed Layer

* Node-to-node communication
* Query distribution across nodes
* Basic replication strategy

### 5. API Layer

* HTTP/gRPC interface for external interaction
* Accepts queries and returns results

### 6. Observability Layer (Web Dashboard)

* Visualizes query execution
* Displays node communication
* Shows performance metrics

---

## Tech Stack

* C++ → Storage engine
* Rust → Distributed layer
* FFI → Communication between C++ and Rust
* Web (TBD) → Dashboard and visualization
* Optional: LLVM (future exploration)

---

## Example Query Flow

1. User sends query:

   ```
   SELECT * FROM users WHERE age > 25;
   ```

2. Query is:

   * Tokenized
   * Parsed into AST
   * Converted into execution plan

3. Execution engine:

   * Translates plan into instructions
   * Runs instructions via VM

4. Distributed layer:

   * Sends tasks to nodes
   * Aggregates results

5. Result is returned and visualized

---

## Milestones

### Phase 1: Single Node Execution

* [ ] Basic query parser
* [ ] AST representation
* [ ] Execution engine (VM)
* [ ] In-memory storage

### Phase 2: Persistent Storage

* [ ] Disk-backed storage
* [ ] Basic indexing
* [ ] Serialization

### Phase 3: Distributed Execution

* [ ] Node communication
* [ ] Query distribution
* [ ] Result aggregation

### Phase 4: Observability

* [ ] API endpoints
* [ ] Web dashboard
* [ ] Execution visualization

### Phase 5: Optimization (Optional)

* [ ] Query optimization
* [ ] Caching
* [ ] Improved indexing

---

## Design Principles

* Build minimal working systems first
* Prefer clarity over cleverness
* Evolve the system incrementally
* Avoid premature optimization
* Never rewrite from scratch — refactor instead

---

## Project Structure (Tentative)

```
/query        -> parser, AST, planner
/vm           -> execution engine
/storage      -> storage engine (C++)
/network      -> distributed layer (Rust)
/api          -> external interface
/dashboard    -> web UI
/docs         -> design notes and decisions
```

---

## Current Status

Early development — core components are being built and integrated incrementally.

---

## Learning Objectives

This project aims to develop a deep understanding of:

* How databases execute queries
* How distributed systems coordinate work
* How low-level systems interact across languages
* How abstractions emerge from implementation

---

## Future Ideas

* Query optimization strategies
* Fault tolerance
* Distributed consensus (Raft/Paxos)
* Custom query language extensions

---

## License

TBD

---

## Notes

This is a long-term project focused on **depth over speed**.
The goal is to build understanding through implementation, not just completion.
