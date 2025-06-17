# Building Pangalactic

Users interact with pangalactic through a set of *binaries*:

- `pg` - The primary high-level revision control binary; users who only want typical revision control can just use this.
- `pg-revcon` - A lower-level utility for revision control, useful for scripting or automation.
- `pg-store` - A *Store* interface, for users who want to interact with the Store directly instead of doing revision control.
- `pg-derive` - A tool to derive deterministic computations within the Store.
- `pg-seed` - A tool for initializing the Store.

Additionally, [this book](../index.md) is developed alongside the code for the binaries.

These binaries are build from a rust workspace [^1].

## Build Steps

The primary build and installation approach is via [`nix`](https://nixos.org/learn/). This handles the entire build and installation process, including building binaries, building WASM components, rendering this book, generating diagrams, and so forth. It can also be used to run the full quality assurance checks performed by the Continuous Integration system. If you prefer not to use `nix` and to build/test the components "by hand", head over to the developer section on [Development without `nix`](../dev/without-nix.md).

1. Install [`nix`](https://nixos.org/download/#download-nix) on your system.
2. Set `export NIX_EXPERIMENTAL=flakes` in your shell to support the newer `flake` feature of `nix`. [^2]
3. Retrieve the source code from [the pangalactic project on Github](https://github.com/nejucomo/pangalactic).
4. In the source code directory run `nix build`.
5. This creates a symlink called `result` containing the binaries, book, and other artifacts:

- `result/bin/` - A directory with the binaries.
- `result/doc/pangalactic/index.html` - The rendering of this book.

[^1]: There are many rust crates within this rust workspace which can enable rust developers to extend and build upon pangalactic. If you are interested in extending or building rust code atop pangalactic, head over to the [Let's Hack!](../dev/lets-hack.md) chapter.

[^2]: The developers of pangalactic consider it a bug of `nix` that this is not enabled by default.
