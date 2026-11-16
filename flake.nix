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
        ];
  in
  {
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
        export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsetting
        s-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
        # Work around blank window / EGL issues with WebKitGTK on some NixOS setups
        export WEBKIT_DISABLE_DMABUF_RENDERER=1
        PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
        PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
        '';
    };
  });
}
