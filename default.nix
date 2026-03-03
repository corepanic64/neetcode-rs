{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "neetcode solutions";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

}
