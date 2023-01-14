use std::sync::{Arc, Mutex};
use rodio::Sink;
use crate::synth::synth_source::{SynthInput};

pub(crate) struct SynthChannel {
    pub(crate) enabled: bool,
    pub(crate) sink: Sink,
    pub(crate) input: Arc<Mutex<SynthInput>>
}