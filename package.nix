{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "aaas";
  inherit ((lib.importTOML ./Cargo.toml).package) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "agg-1.5.0" = "sha256-bCE59NeITaCwgajgyXgP6jxtV7aPihPaZ/Uzh39Po1k=";
    };
  };
}
