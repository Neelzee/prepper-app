{
  description = "Prepper-app dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        libraries = with pkgs; [
          webkitgtk_4_1
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
          libsoup_3
          libayatana-appindicator
          postgresql
        ];

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        prepper-backend = rustPlatform.buildRustPackage {
          pname = "prepper-backend";
          version = "0.1.0";
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./prepper-backend/Cargo.toml
              ./prepper-backend/src
              ./prepper-backend/templates
              ./printer-agent/Cargo.toml
              ./printer-agent/src
              ./app/src-tauri/Cargo.toml
              ./app/src-tauri/src
              ./app/src-tauri/build.rs
              ./migrations
            ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "prepper-backend" ];
          doCheck = false;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [ libpq openssl zstd ];
        };
  in
  { 
    packages = {
      inherit prepper-backend;
      docker-image = pkgs.dockerTools.buildLayeredImage {
        name = "ghcr.io/neelzee/prepper-backend";
        tag = "latest";
        contents = [ prepper-backend pkgs.cacert pkgs.dockerTools.fakeNss ];
        extraCommands = ''
          mkdir -p app/static
          mkdir -p app/data
        '';
        config = {
          Cmd = [ "${prepper-backend}/bin/prepper-backend" ];
          WorkingDir = "/app";
          ExposedPorts."8888/tcp" = {};
          Volumes."/app/static" = {};
          Env = [ "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ];
        };
      };
    };

    devShells.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
        gobject-introspection
        wrapGAppsHook3
        rustToolchain
        nodejs_22
        cargo-tauri
        pnpm
        tailwindcss_4
        esbuild
        openapi-generator-cli
        diesel-cli
        playwright-driver.browsers
        just
        openssl
      ];

      buildInputs = libraries;

      shellHook = ''
        export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH"
        export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
        # Work around blank window / EGL issues with WebKitGTK on some NixOS setups
        export WEBKIT_DISABLE_DMABUF_RENDERER=1
        export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD="1"
        export PLAYWRIGHT_BROWSERS_PATH="${pkgs.playwright-driver.browsers}"
      '';
    };
  });
}
