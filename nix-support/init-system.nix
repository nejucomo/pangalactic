inputs: system:
let
  lib = import ./lib (inputs // { inherit system; });

  vendorDir = lib.crane.vendorCargoDeps { src = lib.self; };
in
{
  packages = {
    default = lib.run-command "pkg-todo" [ ] ''
      echo 'Currently only the `...#book` output is implemented.'
      echo
      echo 'TO DO... implement `nix build`'

      mkdir "$out"
    '';

    inherit vendorDir;

    book = lib.import ./book.nix { inherit vendorDir; };
  };

  devShells.default = lib.import ./dev-shell.nix;
}
