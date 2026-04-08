use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source, cpal::BufferSize, source::Amplify,
};
use std::{fs::File, io::BufReader, sync::mpsc::Receiver};

#[derive(Debug)]
pub enum AudioCmd {
    Mute,
    VolumeUp,
    VolumeDown,
    ChangePart(SoundrackParts),
}

#[derive(Debug)]
pub enum SoundrackParts {
    Calm,
    Buildup,
    Assult,
    Outro,
}

fn load_audio(part: &SoundrackParts) -> Amplify<Decoder<BufReader<File>>> {
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
    let mut handle: MixerDeviceSink = DeviceSinkBuilder::from_default_device()
        .expect("get default device")
        .with_buffer_size(BufferSize::Fixed(4096))
        .open_sink_or_fallback()
        .expect("open device audio stream");
    handle.log_on_drop(false);

    let player: Player = Player::connect_new(&handle.mixer());

    let mut volume: f32 = player.volume();
    player.set_volume(if mute { 0.0 } else { volume });

    // let preloaded_soundtrack_files: [File; 4] = [
    //     File::open("soundtrack/default/test.ogg").expect("open audio file"),
    //     File::open("soundtrack/default/test.ogg").expect("open audio file"),
    //     File::open("soundtrack/default/test.ogg").expect("open audio file"),
    //     File::open("soundtrack/default/test.ogg").expect("open audio file"),
    // ];

    let mut current_part: SoundrackParts = SoundrackParts::Calm;
    player.append(load_audio(&current_part));

    loop {
        for cmd in rx.try_iter() {
            match cmd {
                AudioCmd::Mute => {
                    player.set_volume(if player.volume() == 0.0 { volume } else { 0.0 });
                }
                AudioCmd::VolumeUp => {
                    volume = (volume + 0.1).min(1.0);
                    player.set_volume(volume);
                }
                AudioCmd::VolumeDown => {
                    volume = (volume - 0.1).max(0.0);
                    player.set_volume(volume);
                }
                AudioCmd::ChangePart(part) => {
                    current_part = part;
                    player.append(load_audio(&current_part));
                    player.skip_one();
                }
            }
        }

        if player.empty() {
            player.append(load_audio(&current_part));
        }
    }
}
