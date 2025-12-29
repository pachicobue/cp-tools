{
  description = "競プロ用ツール";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {flake-utils, ...} @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        args = {
          inherit inputs;
          inherit system;
        };
      in {
        nixosConfigurations = import ./nixos-configuration.nix args;
        packages = import ./package.nix args;
        # checks = import ./check.nix args;
        devShells = import ./devshell.nix args;
      }
    );
}
