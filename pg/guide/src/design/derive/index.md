# The Pangalactic Derivation System

The Pangalactic Derivation System is a deterministic execution system built on top of the Pangalactic Store. This enables composable deterministic builds within Pangalactic.

The name `derivation` is largely inspired by the `NixOS` build system, `nix`. It is chosen rather than "build" to emphasize the deterministic nature. In fact, the `pg` revision control application uses the term `build` to refer to more traditional build-like processes which execute on the host system without determinism.
