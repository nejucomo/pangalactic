{
  pname,
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
  cargoPackage ? null, # If present a package name to build
  targetsRgx,
  manifestDir ? ".",
  ...
}:
let
  inherit (cranes.release) buildDepsOnly cargoBuild;

  pnameSuffixDepsOnly = (
    let
      inherit (pkgs.lib.strings) splitString;
      inherit (pkgs.lib.lists) head;
    in
    head (splitString "-" pnameSuffix)
  );

  commonArgs =
    (removeAttrs rawArgs [
      "pnameSuffix"
      "cargoPackage"
      "targetsRgx"
      "manifestDir"
    ])
    // {
      cargoExtraArgs = "--offline --target-dir=target/ --manifest-path ${manifestDir}/Cargo.toml";
    };

  cargoArtifacts = buildDepsOnly (commonArgs // { pname = "${pname}-${pnameSuffixDepsOnly}"; });

  cargoBuilt = cargoBuild (
    commonArgs
    // {
      pname = "${pname}-${pnameSuffix}";
      inherit cargoArtifacts;
      installCargoArtifactsMode = "use-symlink";
      cargoExtraArgs = (
        let
          pkgArgs = if isNull cargoPackage then "" else "--package ${cargoPackage}";
        in
        "${commonArgs.cargoExtraArgs} ${pkgArgs}"
      );
    }
  );

  outputs = run-command "${pnameSuffix}-select-targets" [ pkgs.fd ] ''
    targetDir='${cargoBuilt}/target'
    echo 'Selecting "${targetsRgx}" from:' "$targetDir"
    mkdir "$out"
    fd '${targetsRgx}' "$targetDir" --full-path --exec ln -s '{}' "$out/"
  '';
in
{
  cargo = {
    artifacts = cargoArtifacts;
    build = cargoBuilt;
  };
  inherit outputs;
}
