# Implementation

There are separate commandline tools for accessing the different architectural layers of pangalactic (see [Architecture](../design/arch.md)).

Pangalactic is implemented primarily with `rust`, with `nix` orchestrating the build process. Deterministic computation relies on an assumption that `WASM` is deterministic [^1], and `wasmtime` is currently used as the execution environment. Both "host" code and "WASM guest" code is implemented in rust. Some crates are built both on the target host architecture and the WASM architecture to share data types across that boundary.

[^1]: This needs careful investigation to ensure it is the case. The WASM specifications and issue trackers are not explicit about this goal. In any case, we hope to replace WASM with a system supporting concise non-interactive proofs of computational integrity.
