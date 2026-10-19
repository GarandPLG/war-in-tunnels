use rodio::Decoder;
use std::{
    fs::{self, read},
    io::{BufReader, Cursor},
    path::PathBuf,
    process::{Command, Output},
};
use war_in_tunnels::app::threads::{SoundrackParts, Soundtrack, load_audio};

#[test]
fn audio_repeats_after_end_of_file() {
    let data = read("soundtracks/default/calm.ogg").unwrap();
    let sample_count = Decoder::new(BufReader::new(Cursor::new(data)))
        .unwrap()
        .count();
    assert!(sample_count > 0);

    let mut source = load_audio(&Soundtrack::Default, &SoundrackParts::Calm);
    assert_eq!(source.by_ref().take(sample_count).count(), sample_count);
    assert_eq!(source.take(1024).count(), 1024);
}

#[test]
fn packaged_audio_loads_without_project_working_directory() -> () {
    if std::env::var_os("WAR_IN_TUNNELS_PACKAGED_AUDIO_TEST").is_some() {
        let mut source: rodio::source::Amplify<
            rodio::decoder::LoopedDecoder<BufReader<Cursor<Vec<u8>>>>,
        > = load_audio(&Soundtrack::Default, &SoundrackParts::Calm);
        assert_eq!(source.by_ref().take(1024).count(), 1024);
        return;
    }

    let directory: PathBuf = std::env::temp_dir().join(format!(
        "war-in-tunnels-packaged-audio-{}",
        std::process::id()
    ));
    fs::create_dir_all(directory.join("soundtracks/default")).unwrap();
    fs::create_dir_all(directory.join("working-directory")).unwrap();
    let executable: PathBuf = directory.join("audio-test");
    fs::copy(std::env::current_exe().unwrap(), &executable).unwrap();
    for filename in ["calm.ogg", "test.ogg"] {
        fs::copy(
            PathBuf::from("soundtracks/default").join(filename),
            directory.join("soundtracks/default").join(filename),
        )
        .unwrap();
    }
    let output: Output = Command::new(executable)
        .args([
            "--exact",
            "packaged_audio_loads_without_project_working_directory",
            "--nocapture",
        ])
        .env("WAR_IN_TUNNELS_PACKAGED_AUDIO_TEST", "1")
        .current_dir(directory.join("working-directory"))
        .output()
        .unwrap();
    fs::remove_dir_all(directory).unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
