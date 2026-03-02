{
  description = "Basic rust flake for development and production";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        fenix.follows = "fenix";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      fenix,
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      rustToolchain = fenix.packages.${system}.stable.toolchain;
      naerskLib = pkgs.callPackage naersk {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      packageName = cargoToml.package.name;
      packageVersion = cargoToml.package.version;
      # you can list your other dependencies
      # [dependency_name]Version = cargoToml.dependencies.[dependency_name];
      # ...
    in
    {
      packages.${system} = {
        default = pkgs.callPackage ./default.nix {
          packageName = packageName;
        };

        develop = naerskLib.buildPackage {
          name = "${packageName}";
          src = ./.;
          # buildInputs = with pkgs; [];  <-- add nixpkgs if your dependencies requires system libraries
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        buildInputs = [
          rustToolchain
          # pkgs. ... <-- add nixpkgs if your dependencies requires system libraries
        ];
        nativeBuildInputs = with pkgs; [ pkg-config ];

        shellHook = ''
          echo "${packageName}    v${packageVersion}"
          echo "[you can list your other dependencies here]"
          echo ""
          echo "Commands:"
          echo "    nix build            - Build production version"
          echo "    nix run              - Run production version"
          echo "    nix build .#develop  - Build development version"
          echo "    nix run .#develop    - Run development version"
        '';
      };
    };
}
