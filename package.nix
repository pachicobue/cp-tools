{
  inputs,
  system,
}: let
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [(import inputs.rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
  rootCargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  coreCargoToml = builtins.fromTOML (builtins.readFile ./cpt-core/Cargo.toml);
  extraCargoToml = builtins.fromTOML (builtins.readFile ./cpt-extra/Cargo.toml);
in {
  default = rustPlatform.buildRustPackage {
    pname = coreCargoToml.package.name;
    version = rootCargoToml.workspace.package.version;
    cargoLock = {
      lockFile = ./Cargo.lock;
      allowBuiltinFetchGit = true;
    };
    src = ./.;
    buildType = "release";
    buildAndTestSubdir = "cpt-core";
  };
  extra = rustPlatform.buildRustPackage {
    pname = extraCargoToml.package.name;
    version = extraCargoToml.package.version;
    cargoLock = {
      lockFile = ./Cargo.lock;
      allowBuiltinFetchGit = true;
    };
    src = ./.;
    buildType = "release";
    buildAndTestSubdir = "cpt-extra";
    doCheck = false;
  };
}
