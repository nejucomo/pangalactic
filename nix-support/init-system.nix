inputs: system:
let
  inherit (builtins.import ./lib inputs system)
    import
    self
    pname
    pkgs
    cranes
    run-command
    build-workspace
    combine-derivations
    ;

  crane = cranes.release;

  src = crane.cleanCargoSource self;

  cargoVendorDir = crane.vendorMultipleCargoDeps {
    cargoLockList = [
      (src + "/Cargo.lock")
      (src + "/seed/guests/Cargo.lock")
    ];
  };

  wasm = build-workspace {
    inherit src cargoVendorDir;
    pnameSuffix = "wasm";
    targetsRgx = "release/[^/]+\.wasm$";
    manifestDir = "seed/guests";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  };

  bin = build-workspace {
    inherit src cargoVendorDir;
    pnameSuffix = "bin";
    targetsRgx = "release/pg(-[a-z-]+)?$";
  };

  book = import ./book.nix { inherit cargoVendorDir; };

  install = run-command "install" [ ] ''
    function install-dir-link
    {
      local target="$1"
      local link="$2"

      mkdir -p "$(dirname "$link")"
      ln -vs "$target" "$link"
    }

    install-dir-link '${bin.outputs}' "$out/bin"
    install-dir-link '${wasm.outputs}' "$out/lib/${pname}/wasm"
    install-dir-link '${book}' "$out/doc/${pname}"
  '';

  # All output packages _except_ default and all:
  base-packages = {
    bin-cargo-artifacts = bin.cargo.artifacts;
    bin-cargo-build = bin.cargo.build;
    bin = bin.outputs;

    wasm-cargo-artifacts = wasm.cargo.artifacts;
    wasm-cargo-build = wasm.cargo.build;
    wasm = wasm.outputs;

    inherit book install;
  };

  all = combine-derivations base-packages;
in
{
  packages = base-packages // {
    inherit all;
    default = install;
  };

  devShells.default = import ./dev-shell.nix;
}
