use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::Amplify};
use std::{fs::File, io::BufReader, sync::mpsc::Receiver};

#[derive(Debug)]
pub enum AudioCmd {
    Mute,
    VolumeUp,
    VolumeDown,
}

pub enum SoundrackParts {
    Calm,
    Buildup,
    Assult,
    Outro,
}

fn load_audio(part: SoundrackParts) -> Amplify<Decoder<BufReader<File>>> {
    let file: File = match part {
        SoundrackParts::Calm => File::open("soundtrack/default/test.ogg").expect("open audio file"),
        SoundrackParts::Buildup => {
            File::open("soundtrack/default/test.ogg").expect("open audio file")
        }
        SoundrackParts::Assult => {
            File::open("soundtrack/default/test.ogg").expect("open audio file")
        }
        SoundrackParts::Outro => {
            File::open("soundtrack/default/test.ogg").expect("open audio file")
        }
    };

    Decoder::try_from(file)
        .expect("decoder issue")
        .amplify(0.20)
}

pub fn handle_audio(rx: Receiver<AudioCmd>, mute: bool) {
    let mut handle: MixerDeviceSink =
        DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    handle.log_on_drop(false);

    let player: Player = Player::connect_new(&handle.mixer());

    let mut volume: f32 = player.volume();

    if mute {
        player.set_volume(0.0);
    } else {
        player.set_volume(volume);
    }

    player.append(load_audio(SoundrackParts::Calm));

    loop {
        if player.empty() {
            player.append(load_audio(SoundrackParts::Calm));
        }

        for cmd in rx.try_iter() {
            match cmd {
                AudioCmd::Mute => {
                    if player.volume() == 0.0 {
                        player.set_volume(volume);
                    } else {
                        player.set_volume(0.0);
                    }
                }
                AudioCmd::VolumeUp => {
                    volume = (volume + 0.1).min(1.0);
                    player.set_volume(volume);
                }
                AudioCmd::VolumeDown => {
                    volume = (volume - 0.1).max(0.0);
                    player.set_volume(volume);
                }
            }
        }
    }
}
