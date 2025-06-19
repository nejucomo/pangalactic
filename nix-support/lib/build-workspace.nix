{
  pkgs,
  cranes,
  run-command,
  ...
}:
rawArgs@{
  src,
  cargoVendorDir,

  # Our own custom params:
  pnameSuffix,
  targetsRgx,
  manifestDir ? ".",
  ...
}:
let
  inherit (cranes.release) buildDepsOnly cargoBuild;

  commonArgs =
    (removeAttrs rawArgs [
      "pnameSuffix"
      "targetsRgx"
      "manifestDir"
    ])
    // {
      pname = "${pname}-${pnameSuffix}";
      cargoExtraArgs = "--offline --target-dir=target/ --manifest-path ${manifestDir}/Cargo.toml";
    };

  cargoArtifacts = buildDepsOnly commonArgs;

  cargoBuilt = cargoBuild (
    commonArgs
    // {
      inherit cargoArtifacts;
      installCargoArtifactsMode = "use-symlink";
    }
  );

in
run-command "${pnameSuffix}-select-targets" [ pkgs.fd ] ''
  targetDir='${cargoBuilt}/target'
  echo 'Selecting "${targetsRgx}" from:' "$targetDir"
  mkdir "$out"
  fd '${targetsRgx}' "$targetDir" --full-path --exec ln -s '{}' "$out/"
''
