{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "mimi-ime";
  version = "1.1-unstable";

  src = ../.;
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.autoPatchelfHook
    pkgs.makeWrapper
  ];
  buildInputs = [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.vulkan-loader
    pkgs.stdenv.cc.cc.lib
  ];

  runtimeDependencies = [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.vulkan-loader
  ];

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  postInstall = ''
    wrapProgram $out/bin/mimi-ime \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [
        pkgs.wayland
        pkgs.libxkbcommon
        pkgs.vulkan-loader
      ]}

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
