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
        buildService = { pname, cargoToml, includeStatic ? false }:
          pkgs.rustPlatform.buildRustPackage {
            inherit pname;
            version = "0.1.0";
            src = ./.;
            
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            
            buildAndTestSubdir = cargoToml;
            
            nativeBuildInputs = commonBuildInputs;
            
            buildInputs = [ pkgs.openssl ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            
            # Include static files if needed
            postInstall = if includeStatic then ''
              mkdir -p $out/static
              if [ -d static ]; then
                cp -r static/* $out/static/
              fi
            '' else "";
          };
      in
      {
        packages = {
          web-service = buildService {
            pname = "web-service";
            cargoToml = "services/web-service";
            includeStatic = true;
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