{
  description = "Mimi IME flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs ({ inherit system; });
  in
  with pkgs;
  {
    devShells.${system}.default = mkShell {
      buildInputs = [
        cargo
        cargo-edit
        rustc
        clippy
        pkg-config
        libxkbcommon
        wayland
        vulkan-loader
        mesa
      ];
      PKG_CONFIG_PATH = "${wayland}/lib/pkgconfig:${libxkbcommon}/lib/pkgconfig";
      LD_LIBRARY_PATH = lib.makeLibraryPath [
        wayland
        libxkbcommon
        vulkan-loader
        mesa
      ];
    };

    packages.${system} = {
      default = callPackage ./nix/default.nix {};
    };

    nixosModules."mimi-ime"       = import ./nix/module.nix;
    homeManagerModules."mimi-ime" = import ./nix/module.nix;
  };
}
