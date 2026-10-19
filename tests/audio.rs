use rodio::Decoder;
use std::{
  fs::read,
  io::{BufReader, Cursor},
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
