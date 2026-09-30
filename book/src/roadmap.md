# Roadmap

- iterate on docs to update any conceptual or terminological changes, streamline flow, and track which areas of code much be altered to reconcile with docs.
- rename `Store`/`store` -> `Vault`/`vault`
- CI caching cleanup: consolidate `ci.yaml`+`gh-caching.yml`+`nix-cache.sh` into a single `qaci.yaml`-driven flow w/ `ghci-tool.sh` for LIFO hook cleanup (see branch `ci-caching-cleanup`)
- produce a `v0.1` MVP release design/roadmap which rips out `derive` system altogether to make a complete and working data model first.
- `derive` hostapi call.
  - introduce `sequence` std guest.
- attestation caching
- planset concurrency support
- ipfs store
- pub/sub
- API docs
- revision control tool MVP
- hostenv
- content encryption
- garbage collection?
- self-hosted revision control
- self-hosted compiler
- expand across the galaxy
