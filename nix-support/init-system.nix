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

  seed-dir = run-command "seed-dir" [ ] ''
    outbin="$out/bin"
    outbintest="$out/bin/test"
    mkdir -p "$outbintest"
    for wasm in '${wasm.outputs}'/*
    do
      outrel="$(basename "$wasm" | sed 's|\.wasm$||; s|^test_|test/|')"
      ln -sv "$wasm" "$outbin/$outrel"
    done
  '';

  pg-install-seed = import ./pg-install-seed.nix { inherit bin seed-dir; };

  seed-config = run-command "seed-toml" [ ] ''
    ( set -x
    cat | tee "$out" <<EOF
    seed = "$('${pg-install-seed}' --dirdb './dirdb')"
    EOF
    )
  '';

  merged-bin = run-command "merged-bin" [ ] ''
    mkdir "$out"
    ln -sv '${bin.outputs}'/* "$out/"
    ln -sv '${pg-install-seed}' "$out/pg-install-seed"
  '';

  book = import ./book.nix { inherit cargoVendorDir; };

  install = run-command "install" [ ] ''
    function install-dir-link
    {
      local target="$1"
      local link="$2"

      mkdir -p "$(dirname "$link")"
      ln -vs "$target" "$link"
    }

    install-dir-link '${merged-bin}' "$out/bin"
    install-dir-link '${seed-config}' "$out/etc/${pname}/seed.toml"
    install-dir-link '${seed-dir}' "$out/lib/${pname}/seed"
    install-dir-link '${book}' "$out/doc/${pname}/book"
    install-dir-link '${bin.apidocs}' "$out/doc/${pname}/api"
  '';

  # All output packages _except_ default and all:
  base-packages = {
    inherit book install seed-dir;

    bin-cargo-artifacts = bin.cargo.artifacts;
    bin-cargo-build = bin.cargo.build;
    bin = bin.outputs;

    wasm-cargo-artifacts = wasm.cargo.artifacts;
    wasm-cargo-build = wasm.cargo.build;
    wasm = wasm.outputs;
  };

  all = combine-derivations base-packages;

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
