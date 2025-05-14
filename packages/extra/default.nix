{
  inputs,
  system,
  ...
}:
let
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [ (import inputs.rust-overlay) ];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
  cargoToml = builtins.fromTOML (builtins.readFile ../../cpt-extra/Cargo.toml);
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  cargoLock = {
    lockFile = ../../Cargo.lock;
    allowBuiltinFetchGit = true;
  };
  src = ../../.;
  buildType = "release";
  buildAndTestSubdir = "cpt-extra";
  doCheck = false;
}
