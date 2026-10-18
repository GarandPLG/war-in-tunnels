use clap::ValueEnum;
use once_cell::sync::Lazy;
use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source, cpal::BufferSize,
    decoder::LoopedDecoder, source::Amplify,
};
use std::{
    collections::HashMap,
    fs::read,
    io::{BufReader, Cursor},
    process::exit,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, mpsc::Receiver},
};

#[derive(Debug)]
pub enum AudioCmd {
    Mute,
    VolumeUp,
    VolumeDown,
    ChangePart(SoundrackParts),
    ChangeSoundtrack(Soundtrack),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, ValueEnum)]
pub enum Soundtrack {
    Default,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum SoundrackParts {
    Calm,
    Buildup,
    Assault,
    Outro,
}

type SoundtrackCache = HashMap<SoundrackParts, Vec<u8>>;

static PRELOADED_SOUNDTRACKS: Lazy<RwLock<HashMap<Soundtrack, Arc<SoundtrackCache>>>> =
    Lazy::new(|| {
        let mut map: HashMap<Soundtrack, Arc<HashMap<SoundrackParts, Vec<u8>>>> = HashMap::new();

        map.insert(
            Soundtrack::Default,
            Arc::new(load_soundtrack_folder("default")),
        );

        RwLock::new(map)
    });

fn load_soundtrack_folder(folder: &str) -> SoundtrackCache {
    let base: String = format!("soundtracks/{}", folder);
    let mut cache: HashMap<SoundrackParts, Vec<u8>> = HashMap::new();

    macro_rules! load {
        ($part:ident, $file:expr) => {
            let path = format!("{}/{}", base, $file);
            cache.insert(
                SoundrackParts::$part,
                read(&path).expect(&format!("cannot read {}", path)),
            );
        };
    }

    load!(Calm, "calm.ogg"); // calm.ogg
    load!(Buildup, "test.ogg"); // buildup.ogg
    load!(Assault, "test.ogg"); // assault.ogg
    load!(Outro, "test.ogg"); // outro.ogg

    cache
}

pub fn load_audio(
    soundtrack: &Soundtrack,
    part: &SoundrackParts,
) -> Amplify<LoopedDecoder<BufReader<Cursor<Vec<u8>>>>> {
    {
        let guard: RwLockReadGuard<'_, HashMap<Soundtrack, Arc<HashMap<SoundrackParts, Vec<u8>>>>> =
            PRELOADED_SOUNDTRACKS
                .read()
                .map_err(|e| {
                    eprintln!("RwLock poisoned: {}", e);
                    exit(1);
                })
                .expect("lock issue");

        if let Some(soundtrack_cache) = guard.get(soundtrack) {
            let data: &Vec<u8> = soundtrack_cache.get(part).expect("part missing in cache");

            return Decoder::new_looped(BufReader::new(Cursor::new(data.clone())))
                .expect("decoder issue")
                .amplify(0.20);
        }
    }

    {
        let mut guard: RwLockWriteGuard<
            '_,
            HashMap<Soundtrack, Arc<HashMap<SoundrackParts, Vec<u8>>>>,
        > = PRELOADED_SOUNDTRACKS
            .write()
            .map_err(|e| {
                eprintln!("RwLock poisoned: {}", e);
                exit(1);
            })
            .expect("lock issue");

        if let Some(soundtrack_cache) = guard.get(soundtrack) {
            let data: &Vec<u8> = soundtrack_cache.get(part).expect("part missing in cache");

            return Decoder::new_looped(BufReader::new(Cursor::new(data.clone())))
                .expect("decoder issue")
                .amplify(0.20);
        }

        let folder_name: &str = match soundtrack {
            Soundtrack::Default => "default",
        };

        let new_cache: Arc<HashMap<SoundrackParts, Vec<u8>>> =
            Arc::new(load_soundtrack_folder(folder_name));

        let data: &Vec<u8> = new_cache.get(&part).expect("part missing after load");

        guard.insert(*soundtrack, Arc::clone(&new_cache));

        Decoder::new_looped(BufReader::new(Cursor::new(data.clone())))
            .expect("decoder issue")
            .amplify(0.20)
    }
}

pub fn handle_audio(rx: Receiver<AudioCmd>, mute: bool, soundtrack: Soundtrack) {
    let mut handle: MixerDeviceSink = DeviceSinkBuilder::from_default_device()
        .expect("get default device")
        .with_buffer_size(BufferSize::Fixed(4096))
        .open_sink_or_fallback()
        .expect("open device audio stream");
    handle.log_on_drop(false);

    let player: Player = Player::connect_new(&handle.mixer());

    let mut volume: f32 = player.volume();
    player.set_volume(if mute { 0.0 } else { volume });

    let mut current_soundtrack: Soundtrack = soundtrack;
    let mut current_part: SoundrackParts = SoundrackParts::Calm;
    player.append(load_audio(&current_soundtrack, &current_part));

    for cmd in rx {
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
                player.append(load_audio(&current_soundtrack, &current_part));
                player.skip_one();
            }
            AudioCmd::ChangeSoundtrack(st) => {
                current_soundtrack = st;
                player.append(load_audio(&current_soundtrack, &current_part));
                player.skip_one();
            }
        }
    }
}
