use once_cell::sync::Lazy;
use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source, cpal::BufferSize, source::Amplify,
};
use std::{
    collections::HashMap,
    fs::read,
    io::{BufReader, Cursor},
    sync::mpsc::Receiver,
};

#[derive(Debug)]
pub enum AudioCmd {
    Mute,
    VolumeUp,
    VolumeDown,
    ChangePart(SoundrackParts),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SoundrackParts {
    Calm,
    Buildup,
    Assault,
    Outro,
}

static PRELOADED_DEFAULT_SOUNDTRACK: Lazy<HashMap<SoundrackParts, Vec<u8>>> = Lazy::new(|| {
    let mut map: HashMap<SoundrackParts, Vec<u8>> = HashMap::new();

    macro_rules! load {
        ($part:expr, $path:expr) => {
            map.insert($part, read($path).expect(concat!("cannot read ", $path)));
        };
    }

    load!(SoundrackParts::Calm, "soundtrack/default/test.ogg"); // calm.ogg
    load!(SoundrackParts::Buildup, "soundtrack/default/test.ogg"); // buildup.ogg
    load!(SoundrackParts::Assault, "soundtrack/default/test.ogg"); // assault.ogg
    load!(SoundrackParts::Outro, "soundtrack/default/test.ogg"); // outro.ogg

    map
});

fn load_audio(part: &SoundrackParts) -> Amplify<Decoder<BufReader<Cursor<Vec<u8>>>>> {
    let data: &Vec<u8> = PRELOADED_DEFAULT_SOUNDTRACK
        .get(part)
        .expect("preloaded ogg not found");

    let cursor: Cursor<Vec<u8>> = Cursor::new(data.clone());
    let reader: BufReader<Cursor<Vec<u8>>> = BufReader::new(cursor);

    Decoder::new(reader).expect("decoder issue").amplify(0.20)
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
