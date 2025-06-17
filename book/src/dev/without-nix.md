# Development Without `nix`

-a.k.a. the Tedious Way

## `nix` Training Wheels

If you want to run all of the build steps directly, but still benefit from `nix` to set up a complete development environment, you can run `nix develop`. This will drop you into a new shell process pre-configured with all the appropriate build tool releases to run everything directly without `nix` intermediating (beyond the environment setup).

## No `nix` at All

Finally if you want to build without `nix` present at all on your system, you will have to configure your development environment with all of the prerequisites.

In this case, the `flake.nix` and `nix-support/` files are the best documentation for how to install and configure the prerequisite development environment, because that is the literal code exercised by continuous integration. Start with `nix-support/init-system.nix` for the definition of build targets which are defined in `packages` or the development environment (in `devShells.default`).

In broad strokes the prerequisites for a non-nix system are:

- For the binaries:
  - `rustup` (or the specific toolchain specified in `rust-toolchain.toml`)
  - The `cargo`, `clippy`, `rustc`, and `rustfmt` rust toolchain "components".
  - The `wasm32-unknown-unknown` rust toolchain "target" along with the target for your host. [^1]
- For the book:
  - `mdbook`, `cargo-depgraph`, and `graphviz`.

[^1]: None of this book covers cross-compiling specifically, but we note that a benefit of building with `nix` is much easier cross-compilation support.

## Binaries Build Process

... **TODO**: We want to simplify the non-nix and nix build process before writing this section.
