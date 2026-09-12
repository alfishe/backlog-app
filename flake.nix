{
  description = "Backlog - Cross-platform desktop app for Personal Backlog";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          nodejs_22
        ];

        buildInputs = with pkgs; [
          # Tauri dependencies
          webkitgtk_4_1
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
          libappindicator-gtk3
        ] ++ lib.optionals stdenv.hostPlatform.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.CoreFoundation
          darwin.apple_sdk.frameworks.Foundation
          darwin.apple_sdk.frameworks.AppKit
          darwin.apple_sdk.frameworks.WebKit
        ];

      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "backlog-app";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./src-tauri/Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          # Build frontend first
          preBuild = ''
            cd src-tauri
          '';

          meta = with pkgs.lib; {
            description = "Cross-platform desktop app for Personal Backlog";
            homepage = "https://github.com/alfishe/backlog-app";
            license = licenses.gpl3Only;
            maintainers = [];
            platforms = platforms.linux ++ platforms.darwin;
          };
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          shellHook = ''
            echo "Backlog App development shell"
            echo "Run: npm install && npm run dev"
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
