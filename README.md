# pangalactic

## Terminology

- A WASM module defines how to *derive* an `output` from a *plan*.
- Each *schema* is a specific format of DAG.
- A *plan* is a schema specifying an `exec` WASM and an `input` link. This is the sole input to the `derive` API.
- An *attestation* schema contains `plan` and `output` links. This is the sole output of the `derive` API.

## Design Notes

### Directories in Host Layer

Directories are built in a layer above `BlobStore` so that implementing `BlobStore` is simplified. Why not push directories up into the WASM guest layer to further simplify the WASM host API?

If Directories were implemented in guests, they may use different implementations. Perhaps more importantly, they would need to serialize direct `Key` / `Link` data. By keeping directories in the host layer and providing a `Directory`-aware WASM host API, both issues are addressed: all guests are guaranteed to use the same `Directory` convention, and guests never see `Key` / `Link` data directly, only handles to them. This ensures the same guests operate seamlessly with any backend `BlobStore`.

## Roadmap

- `derive` hostapi call.
  - introduce `sequence` std guest.
- attestation caching
- planset concurrency support
- ipfs store
- pub/sub
- rebrand
- API docs
- revision control tool MVP
- hostenv
- content encryption
- garbage collection?
- self-hosted revision control
- self-hosted compiler
- expand across the galaxy
