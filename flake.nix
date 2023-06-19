{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, rust-overlay, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
        name = "qrazy";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustBinaries = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustBinaries;
          rustc = rustBinaries;
        };

        cliPackage = naersk-lib.buildPackage {
          src = ./.;
          cargoBuildOptions = x: x ++ [
            "--package" "${name}_cli"
          ];
        };

        wasmPackage = naersk-lib.buildPackage {
          src = ./.;
          cargoBuildOptions = x: x ++ [
            "--package" "${name}_wasm"
          ];
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          copyLibs = true;
          copyBins = false;
          nativeBuildInputs = with pkgs; [ binaryen ];
          postInstall = ''
            # Optimize Wasm file size
            wasm-opt -Oz -o $out/lib/${name}.wasm $out/lib/${name}_wasm.wasm
            # Cleanup the lib directory
            find $out/lib -type f ! -name "${name}.wasm" -delete
          '';
        };

        sitePackage = pkgs.stdenv.mkDerivation rec {
          name = "example-site";
          src = ./.;
          installPhase = ''
            mkdir -p $out
            cp example-site/* $out/
            cp ${wasmPackage}/lib/* $out/
          '';
        };

        siteServer = pkgs.writeScriptBin "serve" ''
          ${pkgs.python3}/bin/python3 -m http.server --directory ${sitePackage}
        '';
      in
        {
          packages.${name} = cliPackage;
          packages.wasm = wasmPackage;
          packages.site = sitePackage;

          apps.cli = {
            type = "app";
            program = "${cliPackage}/bin/${name}";
          };
          apps.site = {
            type = "app";
            program = "${siteServer}/bin/serve";
          };
          apps.default = self.apps.${system}.site;

          devShell = with pkgs; mkShell {
            buildInputs = [
              rustBinaries # Rust-related binaries (rustc, cargo, clippy, ...)
              binaryen     # Tools for optimizing wasm modules (wasm-* family of executables)
              wabt         # Tools for working with wasm text format (wasm2wat, wat2wasm, ...)
              wasmtime     # For running the wasm module through WASI (wasmtime --invoke <fn> file.wasm [...args])
              python3
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            RUST_LOG = "debug";
          };
        });
}
