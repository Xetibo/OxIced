{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          _module.args.pkgs = import self.inputs.nixpkgs {
            inherit system;
          };
          devShells.default =
            let
              libPath =
                with pkgs;
                lib.makeLibraryPath [
                  libGL
                  libxkbcommon
                  wayland
                  pkg-config
                  libclang
                ];
            in
            pkgs.mkShell {
              inputsFrom = builtins.attrValues self'.packages;
              buildInputs = with pkgs; [
                libGL
                libxkbcommon
                wayland
                pkg-config
                libclang
                glib
                pango
              ];
              packages = with pkgs; [
                cargo
                cargo-watch
                rustc
                rust-analyzer
                clippy
              ];
              LD_LIBRARY_PATH = libPath;
              LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

            };

        };
    };
}
