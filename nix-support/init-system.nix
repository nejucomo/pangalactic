inputs: system:
let
  inherit (builtins.import ./lib inputs system)
    import
    self
    pname
    cranes
    run-command
    build-workspace
    ;

  crane = cranes.release;

  src = crane.cleanCargoSource self;

  cargoVendorDir = crane.vendorMultipleCargoDeps {
    cargoLockList = [
      (src + "/Cargo.lock")
      (src + "/seed/guests/Cargo.lock")
    ];
  };

  wasms = build-workspace {
    inherit src cargoVendorDir;
    pnameSuffix = "wasms";
    targetsRgx = "release/[^/]+\.wasm$";
    manifestDir = "seed/guests";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  };

  bins = build-workspace {
    inherit src cargoVendorDir;
    pnameSuffix = "bins";
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

    install-dir-link '${bins}' "$out/bin"
    install-dir-link '${wasms}' "$out/lib/${pname}/wasm"
    install-dir-link '${book}' "$out/doc/${pname}"
  '';
in
{
  packages = {
    default = install;
    inherit
      bins
      book
      install
      wasms
      ;
  };

  devShells.default = import ./dev-shell.nix;
}
