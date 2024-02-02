{
  description = "exercise-project-jonas";

  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    advisory-db.url = "github:rustsec/advisory-db";
    advisory-db.flake = false;
    noir.url = "github:noir-lang/noir";
  };

  outputs = inputs@{ self, flake-parts, treefmt-nix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "aarch64-darwin" "x86_64-linux" ];
      imports = [
        treefmt-nix.flakeModule
      ];
      perSystem = { config, self', inputs', system, pkgs, lib, ... }:
        let
          toolchain = inputs'.fenix.packages.${system}.fromToolchainFile {
            file = ./zk-node/rust-toolchain.toml;
            sha256 = "sha256-ks0nMEGGXKrHnfv4Fku+vhQ7gx76ruv6Ij4fKZR3l78=";
          };
          rustToolchain = inputs'.fenix.packages.complete.toolchain;
          craneLib = inputs.crane.lib.${system}.overrideToolchain rustToolchain;

          zkNodeAttrs = {
            src = lib.cleanSourceWith {
              src = craneLib.path ./zk-node;
              filter = path: type: craneLib.filterCargoSources path type;
            };
            nativeBuildInputs = with pkgs; [ pkg-config  ];
            buildInputs = with pkgs; [ openssl.dev sqlite ] ++ (lib.optionals (system == "aarch64-darwin") [pkgs.libiconv pkgs.darwin.Security pkgs.darwin.apple_sdk.frameworks.SystemConfiguration]);
          };
        in
        {
          devShells.default = pkgs.mkShell {
            # Rust Analyzer needs to be able to find the path to default crate
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            CIRCUITS_PATH = ./circuits;
            nativeBuildInputs = [
              rustToolchain
              inputs'.noir.packages.nargo
            ]  ++ zkNodeAttrs.nativeBuildInputs ++ zkNodeAttrs.buildInputs;
            shellHook = ''
              if [ "$RUN_SETUP" = "1" ]; then
                ./bash/setup-files.sh
              fi
            '';
          };

          packages = {
            zk-node-deps = craneLib.buildDepsOnly (zkNodeAttrs // {
              pname = "zk-node-deps";
            });
            zk-node =
                let zk-node' =
                    craneLib.buildPackage (zkNodeAttrs // {
                    cargoArtifacts = self'.packages.zk-node-deps;
                    meta.mainProgram = "zk-node";
                    });
                in pkgs.runCommand "zk-node-wrapped" {
                    buildInputs = [ pkgs.makeWrapper ];
                    meta.mainProgram = "zk-node";
                }
                ''
                    mkdir -p $out/bin
                    makeWrapper ${zk-node'}/bin/zk-node $out/bin/zk-node \
                        --set PATH ${pkgs.lib.makeBinPath [ inputs'.noir.packages.nargo ]} \
                        --set CIRCUITS_PATH ${./circuits} \
                '';

            default = self'.packages.zk-node;

            zk-node-docs = craneLib.cargoDoc (zkNodeAttrs // {
              cargoArtifacts = self'.packages.zk-node-deps;
            });
          };

          checks = {
            zk-node-lint = craneLib.cargoClippy (zkNodeAttrs // {
              cargoArtifacts = self'.packages.zk-node-deps;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

            zk-node-coverage-report = craneLib.cargoTarpaulin (zkNodeAttrs // {
              cargoArtifacts = self'.packages.zk-node-deps;
            });

            zk-node-audit = craneLib.cargoAudit {
              inherit (zkNodeAttrs) src;
              advisory-db = inputs.advisory-db;
            };
          };

          treefmt = {
            projectRootFile = ".git/config";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.rustfmt.package = craneLib.rustfmt;
            settings.formatter = { };
          };
        };
    };
}