# Crate Intra-Dependencies

These are several helpful different views of the intra-workspace crate dependencies. The following were generated with `cargo-depgraph` and `graphviz` and all charts use `--workspace-only --dedup-transitive-deps`.

Crates which appear in both the host and guest charts are build for both host native and WASM targets.

## Host Crates

The host crates are those that implement the cli tools.

![Intra-Workspace Host Dependencies](../assets/generated/depgraph-host.svg)

## Guest Crates

The guest crates are those that implement `pangalactic-guest`, the WASM API for derivations.

![Intra-Workspace Guest Dependencies](../assets/generated/depgraph-guest.svg)

## All Crates

Here are all of the crates, including dev/build crates and dependencies:

![Intra-Workspace All Dependencies](../assets/generated/depgraph-all-deps.svg)
