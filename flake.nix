{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        packages.default = rustPlatform.buildRustPackage {
          pname = "keylist";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = [ gtk4 ];
          nativeBuildInputs = [ pkg-config wrapGAppsHook4 ];
          postInstall = ''
            install -Dm644 ${./keylist.desktop} $out/share/applications/keylist.desktop
            install -Dm644 ${./assets/Light/K_Key_Light.png} $out/share/icons/hicolor/256x256/apps/keylist.png
          '';
        };

        devShells.default = mkShell rec {
          buildInputs = [
            rust-bin.stable.latest.default
            pkg-config
            gtk4
          ];
        };
      }) // {
        overlays.default = final: prev: {
          keylist = self.packages.${prev.system}.default;
        };
      };
}