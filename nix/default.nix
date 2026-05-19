{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "mimi-ime";
  version = "1.0-unstable";

  src = ../.;

  buildInputs = [];
  nativeBuildInputs = [];
  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  meta = with lib; {
    homepage = "";
    description = "mimi-ime rust";
    license = licenses.mit;
  };
}
