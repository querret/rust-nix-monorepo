{
  description = "Rust monorepo with Nix flakes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        # Common build inputs for services
        commonBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
        ];

        # Build function for service
        buildService = { pname, cargoToml }:
          pkgs.rustPlatform.buildRustPackage {
            inherit pname;
            version = "0.1.0";
            src = ./.;
            
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            
            buildAndTestSubdir = cargoToml;
            
            nativeBuildInputs = commonBuildInputs;
            
            # Prereq for reqwest/openssl
            buildInputs = [ pkgs.openssl ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };

      in
      {
        packages = {
          web-service = buildService {
            pname = "web-service";
            cargoToml = "services/web-service";
          };
          
          api-service = buildService {
            pname = "api-service";
            cargoToml = "services/api-service";
          };
          
          cli-tool = buildService {
            pname = "cli-tool";
            cargoToml = "services/cli-tool";
          };
          
          # Build all
          default = pkgs.symlinkJoin {
            name = "all-services";
            paths = with self.packages.${system}; [
              web-service
              api-service
              cli-tool
            ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = commonBuildInputs ++ (with pkgs; [
            rust-analyzer
            cargo-watch
          ]);
          
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}