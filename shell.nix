{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  gitflow = gitAndTools.gitflow;
in

mkShell {
  buildInputs = [
    # Build toolchain
    rustup

    # Project dependencies
    pkgconfig
    openssl

    # Other tools
    git
    gitflow
    cargo-outdated
    cargo-watch
  ];
}
