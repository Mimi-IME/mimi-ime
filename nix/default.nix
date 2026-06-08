{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "mimi-ime";
  version = "1.0-unstable";

  src = ../.;

  buildInputs = [
    pkgs.libxkbcommon
  ];
  nativeBuildInputs = [];
  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  postInstall = ''
    # Install icon
    install -Dm644 images/mimi-ime-icon1.svg \
    $out/share/icons/hicolor/scalable/apps/mimi-ime.svg
  '';

  meta = with lib; {
    homepage = "";
    description = "mimi-ime rust";
    license = licenses.mit;
  };
}
