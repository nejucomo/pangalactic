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

  wasmArtifacts = build-workspace {
    inherit src cargoVendorDir;
    pname = "${pname}-wasmArtifacts";
    relpath = "seed/guests";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  };

  wasms = select-targets wasmArtifacts "*.wasm";

  # binaries = build-workspace { inherit pname; };
in
{
  packages = {
    inherit cargoVendorDir wasms wasmArtifacts;

    book = import ./book.nix { inherit cargoVendorDir; };
  };

  devShells.default = import ./dev-shell.nix;
}
