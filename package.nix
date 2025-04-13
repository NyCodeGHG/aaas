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

  cargoHash = "sha256-ZMnSii7VbTo5w92gVdNryyASioSth/hYZ+VfOjcPQOI=";
}
