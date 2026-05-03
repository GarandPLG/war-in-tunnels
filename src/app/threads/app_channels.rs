use crate::app::threads::AudioCmd;
use ratatui::crossterm::event::KeyEvent;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct AppChannels {
    pub input_tx: Sender<KeyEvent>,
    pub input_rx: Receiver<KeyEvent>,

    pub resize_tx: Sender<(u16, u16)>,
    pub resize_rx: Receiver<(u16, u16)>,

    pub tick_tx: Sender<()>,
    pub tick_rx: Receiver<()>,

    pub audio_tx: Sender<AudioCmd>,
    pub audio_rx: Receiver<AudioCmd>,
}

impl AppChannels {
    pub fn new() -> Self {
        let (input_tx, input_rx) = channel::<KeyEvent>();
        let (resize_tx, resize_rx) = channel::<(u16, u16)>();
        let (tick_tx, tick_rx) = channel::<()>();
        let (audio_tx, audio_rx) = channel::<AudioCmd>();

        Self {
            input_tx,
            input_rx,
            resize_tx,
            resize_rx,
            tick_tx,
            tick_rx,
            audio_tx,
            audio_rx,
        }
    }
}
