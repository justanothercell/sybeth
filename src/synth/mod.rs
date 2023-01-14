use std::sync::MutexGuard;
use crate::synth::synth_source::SynthInput;

pub(crate) mod synth_source;
pub(crate) mod waves;
pub(crate) mod channel;

pub(crate) trait Synth {
    fn get(&mut self, time: usize, input: MutexGuard<SynthInput>) -> f32;
}

pub(crate) struct DummySynth;

impl Synth for DummySynth {
    fn get(&mut self, _time: usize, _input: MutexGuard<SynthInput>) -> f32 {
        0.0
    }
}