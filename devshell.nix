{
  inputs,
  system,
}: let
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [(import inputs.rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
  pkgs.mkShell {
    packages = with pkgs; [
      toolchain
      cargo-bundle-licenses
      cargo-machete
      cargo-deny
      taplo
    ];
  }
