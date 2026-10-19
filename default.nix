{
  lib,
  rustPlatform,
  # fetchFromGitea,
  pkg-config,
  alsa-lib,
  versionCheckHook,
  nix-update-script,
}: let
  pname = "war-in-tunnels";
  version = "0.1.0";
in
  rustPlatform.buildRustPackage {
    inherit pname version;

    src = ./.;
    # src = fetchFromGitea {
    #   domain = "gitea.garandplg.com";
    #   owner = "GarandPLG";
    #   repo = "war-in-tunnels";
    #   tag = "v${version}";
    #   hash = lib.fakeHash;
    # };

    cargoHash = "sha256-cXG/SZxmq/w2CFJiaOYtE6IpShkGY2k0Q+E0QCxgY0I=";

    nativeBuildInputs = [pkg-config];
    buildInputs = [alsa-lib];

    nativeInstallCheckInputs = [versionCheckHook];
    doInstallCheck = true;

    passthru.updateScript = nix-update-script {};

    postInstall = ''
      cp -r soundtracks $out/bin
      mv $out/bin/war_in_tunnels $out/bin/war-in-tunnels
    '';

    meta = {
      description = "TUI underground RTS";
      homepage = "https://gitea.garandplg.com/GarandPLG/war-in-tunnels";
      license = lib.licenses.mit;
      platforms = ["x86_64-linux"];
      maintainers = with lib.maintainers; [
        GarandPLG
      ];
      mainProgram = pname;
    };
  }
