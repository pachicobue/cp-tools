{
  system,
  inputs,
}:
let
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [ (import inputs.rust-overlay) ];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {
  # Add build dependencies
  packages = with pkgs; [
    toolchain
    cargo-about
    cargo-machete
    cargo-deny
    taplo
  ];

  # Add environment variables
  env = { };

  # Load custom bash code
  shellHook = ''

  '';
}
