inputs: system:
let
  inherit (builtins.import ./lib inputs system)
    import
    self
    pname
    crane
    run-command
    ;

  src = crane.cleanCargoSource self;
  vendorDir = crane.vendorCargoDeps { inherit src; };
  cargoArtifacts = crane.buildDepsOnly { inherit pname src vendorDir; };
  binaries = crane.buildPackage { inherit pname src; };
in
{
  packages = {
    default = binaries;

    inherit binaries cargoArtifacts vendorDir;

    book = import ./book.nix { inherit vendorDir; };
  };

  devShells.default = import ./dev-shell.nix;
}
