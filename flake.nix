{
  description = "A layer shell program that renders information you would with a bar as your background instead.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        inherit (pkgs) callPackage;
      in {
        formatter = pkgs.alejandra;

        devShells.default = callPackage ./shell.nix {};

        packages = rec {
          default = callPackage ./. {};
          bgar = default;
        };
      };

      # flake = {};
    };
}
