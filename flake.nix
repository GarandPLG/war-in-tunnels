{
  description = "TUI underground game";

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

  outputs = {
    self,
    nixpkgs,
    naersk,
    fenix,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    rustToolchain = fenix.packages.${system}.stable.toolchain;
    naerskLib = pkgs.callPackage naersk {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };

    cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
    packageVersion = cargoToml.package.version;
  in {
    packages.${system} = {
      default = pkgs.callPackage ./default.nix {};

      develop = naerskLib.buildPackage {
        name = "war-in-tunnels";
        src = ./.;
        buildInputs = with pkgs; [alsa-lib];
        nativeBuildInputs = with pkgs; [pkg-config];
        postInstall = ''
          cp -r soundtracks $out/bin
          mv $out/bin/war_in_tunnels $out/bin/war-in-tunnels
        '';
      };
    };

    devShells.${system}.default = pkgs.mkShell {
      env = {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [alsa-lib]);
      };
      buildInputs = with pkgs; [
        rustToolchain
        alsa-lib
      ];
      nativeBuildInputs = with pkgs; [pkg-config];

      shellHook = ''
        echo "War in Tunnels    v${packageVersion}"
        echo ""
        echo "Commands:"
        echo "    nix build            - Build production version"
        echo "    nix run              - Run production version"
        echo "    nix build .#develop  - Build development version"
        echo "    nix run .#develop    - Run development version"
        echo ""
        cargo run -- -h
      '';
    };
  };
}
