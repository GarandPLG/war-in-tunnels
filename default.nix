{
  lib,
  rustPlatform,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  name = "war-in-tunnels";
  pname = "war-in-tunnels";
  version = "0.1.0";

  src = ./.;
  # buildInputs = [ ];
  nativeBuildInputs = [pkg-config];
  cargoHash = lib.fakeHash;

  meta = {
    description = "TUI underground game";
    homepage = "https://gitea.garandplg.com/GarandPLG/war-in-tunnels";
    license = lib.licenses.mit;
    maintainers = [
      "Garand_PLG"
    ];
  };
}
