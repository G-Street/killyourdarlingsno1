{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    wasm-server-runner = {
      url = "github:jakobhellermann/wasm-server-runner";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      wasm-server-runner,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              # Rust dependencies
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })
              pkg-config
              # WASM build / packaging tools
              wasm-pack
              wasm-bindgen-cli_0_2_106
              (wasm-server-runner.packages.${system}.default)
              zip
            ]
            ++ lib.optionals (lib.strings.hasInfix "linux" system) [
              # for Linux
              # Audio (Linux only)
              alsa-lib
              # Cross Platform 3D Graphics API
              vulkan-loader
              # For debugging around vulkan
              vulkan-tools
              # Other dependencies
              libudev-zero
              libx11
              libxcursor
              libxi
              libxrandr
              libxkbcommon
              wayland
              nixfmt
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              libx11
              libxi
              libxcursor
              libxkbcommon
              wayland
            ];
          };
      }
    );
}
