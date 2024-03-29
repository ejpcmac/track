{
  description = "A quick-and-dirty CLI tool for tracking parcels.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk = pkgs.callPackage inputs.naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
      in
      {
        packages = {
          default = self.packages.${system}.track;
          track = naersk.buildPackage { src = ./.; };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Build toolchain.
            rust-toolchain

            # Tools.
            git
            gitAndTools.gitflow
          ];
        };
      });
}
