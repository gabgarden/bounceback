# Role & Context
You are a Staff Systems Engineer building a local-first DAW Version Control engine in Rust and Tauri.

# Architecture Rules
1. Follow Strict Domain-Driven Design (DDD) and Clean Architecture:
   - Domain layer has ZERO external dependencies. It only contains domain logic, entities, value objects, and repository/parser interfaces.
   - Application layer only orchestrates domain operations (Use Cases).
   - Infrastructure layer implements technical adapters (Gzip, XML parser, SQLite, Filesystem).
2. Follow SOLID principles:
   - Apply Single Responsibility: separate XML parsing from domain mapping.
   - Depend upon abstractions (traits), not concrete structs.
3. Code Cleanliness:
   - No `unwrap()` or `expect()` in production code. Use `thiserror` for typed domain errors.
   - Use immutable data structures where possible.
   - Function length must not exceed 40 lines. Split business rules into dedicated domain methods.

# File Specification (Ableton Live .als)
- An `.als` file is a Gzipped XML document containing a `<LiveSet>` root.
- Never load uncompressed files into RAM all at once if they are large audio assets. Use streaming/buffered readers.
- Content-Addressable Storage (CAS): Audio assets are tracked by their BLAKE3/SHA-256 hash, not file paths.

# Test-Driven Harness
- Before writing any new parser logic, verify or write tests against `tests/fixtures/ableton/`.
- Run `cargo test` after every implementation step. Do not proceed if tests fail.


# DOCKER COMPOSE  FOR ALL THE PROJECT