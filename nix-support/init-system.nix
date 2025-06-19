inputs: system:
let
  inherit (builtins.import ./lib inputs system)
    import
    self
    pname
    crane
    build-workspace
    select-targets
    ;

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
in
{
  packages = {
    inherit bins cargoVendorDir wasms;

    book = import ./book.nix { inherit cargoVendorDir; };
  };

  devShells.default = import ./dev-shell.nix;
}
