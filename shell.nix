{ self, pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    alejandra
    nixd
    deadnix
    statix

    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    self.formatter.${system}
  ];
}
