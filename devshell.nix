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
  packages = [
    toolchain
    pkgs.cargo-audit
    pkgs.cargo-bundle-licenses
    pkgs.cargo-deny
    pkgs.cargo-license
    pkgs.taplo
  ];

  # Add environment variables
  env = { };

  # Load custom bash code
  shellHook = ''

  '';
}
