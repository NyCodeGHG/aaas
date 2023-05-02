{
  description = "Asciinema as a Service";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
        fontsConf = pkgs.makeFontsConf {
          fontDirectories = [ pkgs.jetbrains-mono ];
        };

        craneLib = crane.lib.${system};
        aaas = craneLib.buildPackage {
          pname = "aaas";
          src = craneLib.cleanCargoSource (craneLib.path ./.);
        };
        dockerImage = pkgs.dockerTools.buildImage {
          name = "aaas";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ aaas ];
            pathsToLink = [ "/bin" ];
          };
          config = {
            Cmd = [ "/bin/aaas" ];
          };
          runAsRoot = ''
            mkdir -p /etc/fonts
            ln -s ${fontsConf} /etc/fonts/fonts.conf
          '';
        };
      in
      {
        defaultPackage = aaas;
        packages = {
          inherit aaas;
          docker = dockerImage;
        };
        devShell = pkgs.mkShell {
          inputsFrom = [ aaas ];
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            cargo-audit
            cargo-watch
          ];
          shellHook = ''
            export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
          '';
        };
      });
}
