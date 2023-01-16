use std::sync::MutexGuard;
use std::task::ready;
use crate::synth::synth_source::SynthInput;
use crate::synth::waves::{SawSynth, SineSynth, SquareSynth, TriangleSynth};

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

pub(crate) struct Instrument {
    pub(crate) id: u16,
    pub(crate) name: &'static str,
    pub(crate) synth: Box<dyn Synth>
}

pub(crate) const INSTRUMENTS: [u16;5] = [
    0b0000_0000_0000_0000,
    0b0000_0000_0000_0001,
    0b0000_0000_0000_0010,
    0b0000_0000_0000_0011,
    0b0000_0000_0000_0100
];

pub(crate) fn create_instrument(id: u16) -> Instrument {
   let (name, synth): (&str, Box<dyn Synth>) =  match id {
        0b0000_0000_0000_0000 => ("---", Box::new(DummySynth)),
        0b0000_0000_0000_0001 => ("SIN", Box::new(SineSynth)),
        0b0000_0000_0000_0010 => ("SQR", Box::new(SquareSynth)),
        0b0000_0000_0000_0011 => ("SAW", Box::new(SawSynth)),
        0b0000_0000_0000_0100 => ("TRI", Box::new(TriangleSynth)),
        _ => panic!("invalid instrument {id}")
    };
    Instrument {
        id, name, synth
    }
}

