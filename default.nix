{
  lib,
  rustPlatform,
  packageName,
  pkg-config,
# add nixpkgs if your dependencies requires system libraries
}:
rustPlatform.buildRustPackage {
  name = "${packageName}";
  src = ./.;
  # buildInputs = [ ];  <-- add nixpkgs if your dependencies requires system libraries
  nativeBuildInputs = [ pkg-config ];
  cargoHash = lib.fakeHash; # <-- Put your real hash here after failed nix build
}
