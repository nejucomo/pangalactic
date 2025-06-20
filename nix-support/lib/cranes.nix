{
  self,
  pkgs,
  crane,
}:
let
  inherit (builtins) fromTOML mapAttrs readFile;
  inherit (pkgs.lib.trivial) flip pipe;

  path = self + "/rust-toolchain.toml";

  release = pipe path [
    readFile
    fromTOML
    (p: p.toolchain)
  ];

  toolchainParams = {
    inherit release;

    dev = release // {
      components = release.components ++ [
        "rust-analyzer"
        "rust-src"
      ];
    };
  };

  baseCrane = crane.mkLib pkgs;

  overrideCrane = flip pipe [
    pkgs.lib.traceValSeq
    pkgs.rust-bin.fromRustupToolchain
    baseCrane.overrideToolchain
  ];

in
mapAttrs (_: overrideCrane) toolchainParams
