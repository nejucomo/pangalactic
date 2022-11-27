# dagwasm

## Design Notes

### Directories in Host Layer

Directories are built in a layer above `BlobStore` so that implementing `BlobStore` is simplified. Why not push directories up into the WASM guest layer to further simplify the WASM host API?

If Directories were implemented in guests, they may use different implementations. Perhaps more importantly, they would need to serialize direct `Key` / `Link` data. By keeping directories in the host layer and providing a `Directory`-aware WASM host API, both issues are addressed: all guests are guaranteed to use the same `Directory` convention, and guests never see `Key` / `Link` data directly, only handles to them. This ensures the same guests operate seamlessly with any backend `BlobStore`.
