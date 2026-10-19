use clap::{Parser, error::ErrorKind};
use std::process::Command;
use war_in_tunnels::{app::View, cli::Cli};

#[test]
fn defaults_start_in_menu_without_logging() {
    let args = Cli::try_parse_from(["game"]).unwrap();
    assert_eq!(args.view, View::MainMenu);
    assert_eq!((args.map_width, args.map_height), (42, 11));
    assert_eq!(args.username, "Player");
    assert!(!args.log);
    assert!(!args.disable_audio);
    assert!(!args.mute);
}

#[test]
fn numeric_options_accept_boundaries_and_reject_out_of_range_values() {
    for (flag, min, max) in [
        ("--map-width", 36, 108),
        ("--map-height", 11, 39),
        ("--starting-wood", 1, 150),
        ("--starting-metal", 1, 150),
        ("--supply-limit", 49, 149),
        ("--skill-points-limit", 90, 690),
    ] {
        for value in [min, max] {
            assert!(
                Cli::try_parse_from(["game", flag, &value.to_string()]).is_ok(),
                "{flag}={value}"
            );
        }
        for value in [min - 1, max + 1] {
            assert!(
                Cli::try_parse_from(["game", flag, &value.to_string()]).is_err(),
                "{flag}={value}"
            );
        }
    }
}

#[test]
fn xp_modifier_rejects_non_finite_malformed_and_out_of_range_values() {
    for value in ["0.5", "1", "2.0"] {
        let args = Cli::try_parse_from(["game", "--xp-modifier", value]).unwrap();
        assert_eq!(args.xp_modifier, value.parse::<f32>().unwrap());
    }
    for value in ["0.49", "2.01", "NaN", "inf", "text"] {
        assert!(
            Cli::try_parse_from(["game", "--xp-modifier", value]).is_err(),
            "{value}"
        );
    }
}

#[test]
fn enum_options_and_audio_flags_are_parsed() {
    let args =
        Cli::try_parse_from(["game", "--view", "skirmish", "--disable-audio", "--mute"]).unwrap();
    assert_eq!(args.view, View::Skirmish);
    assert!(args.disable_audio && args.mute);
    for flag in [
        "--view",
        "--game-mode",
        "--zoom-level",
        "--perk-deck",
        "--soundtrack",
    ] {
        assert_eq!(
            Cli::try_parse_from(["game", flag, "unknown"])
                .unwrap_err()
                .kind(),
            ErrorKind::InvalidValue
        );
    }
}

#[test]
fn binary_help_and_version_work_without_terminal_or_audio() {
    for (flag, expected) in [("--help", "Usage:"), ("--version", "0.1.0")] {
        let output = Command::new(env!("CARGO_BIN_EXE_war_in_tunnels"))
            .arg(flag)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8(output.stdout).unwrap().contains(expected));
    }
    let output = Command::new(env!("CARGO_BIN_EXE_war_in_tunnels"))
        .args(["--map-width", "0"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("--map-width")
    );
}
