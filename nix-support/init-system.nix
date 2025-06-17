inputs: system:
let
  lib = import ./lib (inputs // { inherit system; });
  inherit (lib) crane run-command;

  src = crane.cleanCargoSource lib.self;
  vendorDir = crane.vendorCargoDeps { inherit src; };

  todoPkg = run-command "pkg-todo" [ ] ''
    echo 'Currently only the `...#book` output is implemented.'
    echo
    echo 'TO DO... implement `nix build`'

    mkdir "$out"
  '';
in
{
  packages = {
    default = todoPkg;

    inherit vendorDir;

    book = lib.import ./book.nix { inherit vendorDir; };
  };

  devShells.default = lib.import ./dev-shell.nix;
}
