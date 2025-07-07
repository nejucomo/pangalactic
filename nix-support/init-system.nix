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
  seed-crates = "seed-crates";

  cargoVendorDir = crane.vendorMultipleCargoDeps {
    cargoLockList = [
      (src + "/Cargo.lock")
      (src + "/${seed-crates}/Cargo.lock")
    ];
  };

  bin = (
    build-workspace {
      inherit src cargoVendorDir;
      pnameSuffix = "bin";
      targetsRgx = "release/pg(-[a-z-]+)?$";

      postBuild = ''
        cargo doc --workspace
      '';
    }
    // {
      apidocs = run-command "bin-apidocs" [ ] ''
        ln -sv '${bin.cargo.build}/target/doc' "$out"
      '';
    }
  );

  wasm = build-workspace {
    inherit src cargoVendorDir;
    pnameSuffix = "wasm";
    targetsRgx = "release/[^/]+\.wasm$";
    manifestDir = "./${seed-crates}";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  };

  seed-dir = import ./seed-dir.nix { inherit wasm; };
  pg-install-seed = import ./pg-install-seed.nix { inherit bin seed-dir; };

  seed-config = run-command "seed-toml" [ ] ''
    ( set -x
    cat | tee "$out" <<EOF
    seed = "$('${pg-install-seed}/pg-install-seed' --dirdb './dirdb')"
    EOF
    )
  '';

  merged-bin = pkgs.buildEnv {
    name = "merged-bin";
    paths = [
      bin.outputs
      pg-install-seed
    ];
  };

  depgraphs = import ./depgraphs.nix { inherit cargoVendorDir; };
  book = import ./book.nix { inherit depgraphs; };

  install = combine-derivations "install" {
    "bin" = merged-bin;
    "etc/${pname}/seed.toml" = seed-config;
    "lib/${pname}/seed" = seed-dir;
    "doc/${pname}/book" = book;
    "doc/${pname}/api" = bin.apidocs;
  };

  # All output packages _except_ default and all:
  base-packages = {
    inherit
      book
      depgraphs
      install
      merged-bin
      seed-config
      seed-dir
      ;

    bin-cargo-artifacts = bin.cargo.artifacts;
    bin-cargo-build = bin.cargo.build;
    bin = bin.outputs;

    wasm-cargo-artifacts = wasm.cargo.artifacts;
    wasm-cargo-build = wasm.cargo.build;
    wasm = wasm.outputs;
  };

  all = combine-derivations "base-packages" base-packages;

  packages = base-packages // {
    inherit all;
    default = install;
  };
in
{
  inherit packages;

  devShells.default = import ./dev-shell.nix;
  checks = import ./checks packages;
}
