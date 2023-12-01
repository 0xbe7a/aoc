{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.nightly."2022-12-06".default.override {
            extensions = [ "rust-src" ];
          })
        ];
        RUSTFLAGS = "-C target-cpu=native";
      };
    }
  );
}
