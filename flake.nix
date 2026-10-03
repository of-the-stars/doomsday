{
  description = "of-the-star's custom rust development flake"; # TODO: Change description for project

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
      ];
      iterOverSystems = nixpkgs.lib.genAttrs systems;
      forSystem =
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          rust-toolchain =
            if builtins.pathExists ./rust-toolchain.toml then
              pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
            else
              pkgs.rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
              };

          # Instantiates custom craneLib using toolchain
          craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

          src = craneLib.cleanCargoSource ./.;
          pname = (craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; }).pname;

          # Common arguments shared between buildPackage and buildDepsOnly
          commonArgs = {
            inherit src;
            strictDeps = true;

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
              openssl
            ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          package = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        in
        {
          inherit
            package
            ;

          "${pname}" = package;

          devShell = pkgs.mkShell {
            # Inherits buildInputs from package
            inputsFrom = [ package ];

            # Additional packages for the dev environment
            packages = with pkgs; [
            ];

            shellHook = "";

            env = {
              # Needed for rust-analyzer
              RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
            };
          };

          formatter = pkgs.nixfmt-tree; # Nix flake formatter. Run `nix fmt` to use
        };
    in
    {
      formatter = iterOverSystems (system: (forSystem system).formatter);

      devShells = iterOverSystems (system: {
        default = (forSystem system).devShell;
      });

      packages = iterOverSystems (system: {
        default = (forSystem system).package;
      });
    };
}
