# Architecture

The Pangalactic Platform is composed of layers:

- The Pangalactic Store - a decentralized, confidential, network-agnostic, and immutable storage system.
- The Pangalactic Pub/Sub System - a decentralized capability system for publishing and retrieving updates to a project.
- The Pangalactic Derivation System - a deterministic computation execution layer with access constrained to the Pangalactic Store. (Note: this layer does not have direct access to Pub/Sub.)
- The `pg` application - A revision control, deterministic build, and dependency management tool.
