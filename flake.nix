{
  description = "Asciinema as a Service";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fontsConf = pkgs.makeFontsConf {
          fontDirectories = [ pkgs.jetbrains-mono ];
        };

        aaas = pkgs.callPackage ./package.nix { };
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
            ExposedPorts = {
              "8080" = { };
            };
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
