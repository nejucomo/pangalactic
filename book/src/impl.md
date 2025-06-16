# Implementation

There are separate commandline tools for accessing the different architectural layers of pangalactic (see [Architecture](/architecture.md)).

Pangalactic is implemented primarily with `rust`, with `nix` orchestrating the build process. Deterministic computation relies on an assumption that `WASM` is deterministic [^1], and `wasmtime` is currently used as the execution environment. Both "host" code and "WASM guest" code is implemented in rust. Some crates are built both on the target host architecture and the WASM architecture to share data types across that boundary.

## Crate Dependencies

Here are the current crate dependencies (with redundant transitive dependencies omitted):

![Intra-Workspace Dependencies](assets/generated/depgraph-ws-dedup.svg)
