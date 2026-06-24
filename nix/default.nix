{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
, rustPlatform ? pkgs.rustPlatform
, enableSettingsUi ? true
}:

rustPlatform.buildRustPackage {
  pname = "mimi-ime";
  version = "1.2-unstable";

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
  ] ++ lib.optionals enableSettingsUi [
    pkgs.libGL  # eframe/wgpu cần
  ];

  runtimeDependencies = [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.vulkan-loader
  ];

  buildFeatures = lib.optionals enableSettingsUi [ "settings-ui" ];

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

    install -Dm644 images/mimi-ime-icon1.svg \
      $out/share/icons/hicolor/scalable/apps/mimi-ime.svg
  '';

  meta = with lib; {
    homepage = "";
    description = "mimi-ime rust";
    license = licenses.mit;
  };
}
